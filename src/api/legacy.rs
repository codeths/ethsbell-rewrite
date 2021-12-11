#![allow(missing_docs)]
use crate::impls::MaxElement;
use crate::schedule::{Period, PeriodType, Schedule, ScheduleType};
use chrono::{Datelike, Local, NaiveDate, NaiveTime, Timelike, Weekday};
use rocket::response::content::Html;
use rocket::{Route, State};
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rocket_okapi::{openapi, routes_with_openapi};
use serde::Serialize;
use serde_json::json;
use std::sync::{Arc, RwLock};

/// Returns a list of all our routes for Rocket.
pub fn routes() -> Vec<Route> {
	routes_with_openapi![display, data, oliver]
}

/// This returns a templated HTML response for use with the original frontend and the browser extension.
#[openapi(skip)]
#[get("/display")]
fn display(schedule: State<Arc<RwLock<Schedule>>>) -> Template {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	};
	let now = Local::now();
	let schedule = schedule.read().unwrap().on_date(now.date().naive_local());
	let period = schedule.0.at_time(now.time());
	let friendly_name = period
		.1
		.iter()
		.map(|v| v.friendly_name.clone())
		.collect::<Vec<String>>()
		.join(", ");
	let friendly_name = match friendly_name.len() {
		0 => "No Period".to_string(),
		_ => friendly_name,
	};
	let next_friendly_name = period
		.2
		.clone()
		.map(|v| v.friendly_name)
		.unwrap_or_else(|| "No Period".to_string());
	let start = period
		.2
		.iter()
		.map(|v| v.start)
		.max_element()
		.map(|v| v.to_string())
		.next()
		.unwrap_or_else(|| "No Time".to_string());
	let end = period
		.1
		.iter()
		.map(|v| v.end)
		.max_element()
		.map(|v| v.to_string())
		.next()
		.unwrap_or_else(|| "No Time".to_string());
	Template::render(
		"legacy-display",
		json!({
			"friendly_name": friendly_name,
			"next_friendly_name": next_friendly_name,
			"start": start,
			"end": end
		}),
	)
}

/// The closest we can get to the original's API with the differences in design between us
#[openapi]
#[get("/data")]
fn data(schedule: State<Arc<RwLock<Schedule>>>) -> Json<LegacySchedule> {
	Json(
		schedule
			.read()
			.unwrap()
			.on_date(Local::now().date().naive_local())
			.0
			.into(),
	)
}

/// A hopefully-compatible replica of the original's response.
#[derive(Serialize, JsonSchema)]
#[allow(non_snake_case)]
struct LegacySchedule {
	/// Today's schedule
	pub schedule: LegacyScheduleKey,
	/// The name of the current period.
	pub theSlot: Option<String>,
	/// The time in seconds since 00:00 January 1st, 1970
	pub time: usize,
	/// The name of the next period.
	pub theNextSlot: Option<String>,
	/// The end of the current period, formatted as H:M
	pub periodEndTime: Option<String>,
	/// The end of the previous period in seconds since 00:00 January 1st, 1970. Might be broken.
	pub endOfPreviousPeriod: usize,
	/// The current date.
	pub formattedDate: String,
	/// The name of the day of the week.
	pub dayOfWeek: String,
	/// The current time, formatted as H:M
	pub formattedTime: String,
	/// Always null.
	pub scheduleCode: Option<String>,
	/// Always false.
	pub isListingForDay: bool,
	/// False if there are no periods.
	pub noSchedule: bool,
	/// True only if there is one or more Class(_) periods.
	pub schoolInSession: bool,
	/// Always "1"
	pub school_id: String,
	/// The class number of the next Class(_) period, or -1 if none exist.
	pub theNextSlot_: String,
	/// The time of day as a negative number of minutes.
	pub timeLeftInPeriod: isize,
	/// The time since the end of the last period in minutes.
	pub timeSinceLastPeriod: isize,
}

/// A container for the schedule's name and the list of its periods.
#[derive(Serialize, JsonSchema)]
#[allow(non_snake_case)]
struct LegacyScheduleKey {
	name: String,
	period_array: Vec<LegacyPeriod>,
}

/// A replica of the original's period representation.
#[derive(Serialize, JsonSchema)]
#[allow(non_snake_case)]
struct LegacyPeriod {
	/// The start of the period, formatted as H:M
	pub start_time: String,
	/// The end of the period, formatted as H:M
	pub end_time: String,
	/// The period kind shown with Debug.
	pub period_notice: Option<String>,
	/// The friendly name of the period.
	pub period_name: String,
}

impl From<ScheduleType> for LegacySchedule {
	fn from(schedule: ScheduleType) -> Self {
		let context = schedule.at_time(Local::now().time());
		let context = [
			context.0,
			match context.1.len() {
				0 => None,
				1 => Some(context.1[0].clone()),
				_ => None,
			},
			context.2,
		];

		let context = context
			.iter()
			.map(|v| match v {
				Some(p) => match p.kind {
					PeriodType::Break | PeriodType::BeforeSchool | PeriodType::AfterSchool => None,
					_ => Some(p.clone()),
				},
				None => None,
			})
			.collect::<Vec<Option<Period>>>();
		LegacySchedule {
			schedule: LegacyScheduleKey {
				name: schedule.friendly_name.clone(),
				period_array: schedule.periods.iter().map(|v| v.clone().into()).collect(),
			},
			theSlot: context[1].clone().map(|v| v.friendly_name),
			time: Local::now().timestamp() as usize,
			theNextSlot: match context[2].clone() {
				Some(v) => match v.kind {
					PeriodType::Class(_) => Some(v.friendly_name),
					_ => schedule.first_class().map(|v| match v.kind {
						PeriodType::Class(_) => v.friendly_name,
						_ => panic!("?!?!"),
					}),
				},
				None => schedule.first_class().map(|v| match v.kind {
					PeriodType::Class(_) => v.friendly_name,
					_ => panic!("?!?!"),
				}),
			},
			periodEndTime: context[1].clone().map(|v| v.end.to_legacy()),
			endOfPreviousPeriod: context[0].clone().map(|v| v.end_timestamp).unwrap_or(0) as usize,
			formattedDate: Local::now().date().naive_local().to_legacy(),
			dayOfWeek: Local::now().weekday().to_legacy(),
			formattedTime: Local::now().time().to_legacy(),
			scheduleCode: None,     // Unclear what this is for
			isListingForDay: false, // Unclear what this is for
			noSchedule: schedule.periods.is_empty(),
			schoolInSession: schedule
				.periods
				.iter()
				.filter(|v| matches!(v.kind, PeriodType::Class(_)))
				.count() > 1,
			school_id: "1".to_string(), // Unclear what this is for
			theNextSlot_: match context[2].clone() {
				Some(v) => match v.kind {
					PeriodType::Class(n) => Some(n.parse().unwrap()),
					_ => schedule.first_class().map(|v| match v.kind {
						PeriodType::Class(n) => n,
						_ => panic!("?!?!"),
					}),
				},
				None => schedule.first_class().map(|v| match v.kind {
					PeriodType::Class(n) => n.parse().unwrap(),
					_ => panic!("?!?!"),
				}),
			}
			.unwrap_or_else(|| "-1".to_string()),
			timeLeftInPeriod: {
				let now = Local::now();
				-(now.hour() as isize * 60 + now.minute() as isize)
			},
			timeSinceLastPeriod: context[0]
				.clone()
				.map(|v| (Local::now().time() - v.end).num_minutes())
				.unwrap_or_else(|| 0) as isize,
		}
	}
}

impl From<Period> for LegacyPeriod {
	fn from(period: Period) -> Self {
		LegacyPeriod {
			start_time: period.start.to_legacy(),
			end_time: period.end.to_legacy(),
			period_notice: Some(format!("{:?}", period.kind)),
			period_name: period.friendly_name,
		}
	}
}

trait ToLegacyFormat {
	fn to_legacy(self) -> String;
}

impl ToLegacyFormat for NaiveTime {
	fn to_legacy(self) -> String {
		format!("{}:{}", self.hour(), self.minute())
	}
}
impl ToLegacyFormat for NaiveDate {
	fn to_legacy(self) -> String {
		format!("{}-{}-{}", self.year(), self.month(), self.day())
	}
}

impl ToLegacyFormat for Weekday {
	fn to_legacy(self) -> String {
		match self {
			chrono::Weekday::Mon => "Monday",
			chrono::Weekday::Tue => "Tuesday",
			chrono::Weekday::Wed => "Wednesday",
			chrono::Weekday::Thu => "Thursday",
			chrono::Weekday::Fri => "Friday",
			chrono::Weekday::Sat => "Saturday",
			chrono::Weekday::Sun => "Sunday",
		}
		.to_string()
	}
}

/// 🪦
#[openapi]
#[get("/oliver")]
fn oliver() -> Html<&'static str> {
	Html(include_str!("oliver.html"))
}
