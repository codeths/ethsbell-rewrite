use std::sync::{Arc, RwLock};

use crate::schedule::Schedule;
use crate::schedule::{Period, PeriodType, ScheduleType};
use chrono::{Datelike, Local};
use rocket::{response::content::Html, Route, State};
use rocket_contrib::json::Json;
use rocket_okapi::{openapi, routes_with_openapi};
use serde::Serialize;

pub fn routes() -> Vec<Route> {
	routes_with_openapi![display, data]
}

/// This returns a templated HTML response for use with the original frontend and the browser extension.
#[openapi]
#[get("/display")]
fn display(schedule: State<Arc<RwLock<Schedule>>>) -> Html<String> {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	};
	let now = Local::now();
	let schedule = schedule
		.read()
		.unwrap()
		.on_date(now.date().naive_local())
		.clone();
	let period = schedule.at_time(now.time()).clone();
	let friendly_name = period[1]
		.clone()
		.map(|v| v.friendly_name)
		.unwrap_or("No Period".to_string());
	let next_friendly_name = period[2]
		.clone()
		.map(|v| v.friendly_name)
		.unwrap_or("No Period".to_string());
	let start = period[1]
		.clone()
		.map(|v| v.start.to_string())
		.unwrap_or("No Time".to_string());
	let end = period[1]
		.clone()
		.map(|v| v.end.to_string())
		.unwrap_or("No Time".to_string());
	Html(format!(
		include_str!("./legacy-display.html"),
		start = start,
		end = end,
		friendly_name = friendly_name,
		next_friendly_name = next_friendly_name
	))
}

#[openapi]
#[get("/data")]
fn data(schedule: State<Arc<RwLock<Schedule>>>) -> Json<LegacySchedule> {
	Json(
		schedule
			.read()
			.unwrap()
			.on_date(Local::now().date().naive_local())
			.clone()
			.into(),
	)
}

#[derive(Serialize, JsonSchema)]
#[allow(non_snake_case)]
struct LegacySchedule {
	pub schedule: LegacyScheduleKey,
	pub theSlot: Option<String>,
	pub time: usize,
	pub theNextSlot: Option<String>,
	pub periodEndTime: Option<String>,
	pub endOfPreviousPeriod: usize,
	pub formattedDate: String,
	pub dayOfWeek: String,
	pub formattedTime: String,
	pub scheduleCode: Option<String>,
	pub isListingForDay: bool,
	pub noSchedule: bool,
	pub schoolInSession: bool,
	pub school_id: String,
	pub theNextSlot_: isize,
	pub timeLeftInPeriod: isize,
	pub timeSinceLastPeriod: isize,
}

#[derive(Serialize, JsonSchema)]
#[allow(non_snake_case)]
struct LegacyScheduleKey {
	name: String,
	period_array: Vec<LegacyPeriod>,
}

#[derive(Serialize, JsonSchema)]
#[allow(non_snake_case)]
struct LegacyPeriod {
	pub start_time: String,
	pub end_time: String,
	pub period_notice: Option<String>,
	pub period_name: String,
}

impl From<ScheduleType> for LegacySchedule {
	fn from(schedule: ScheduleType) -> Self {
		let context = schedule.at_time(Local::now().time());
		LegacySchedule {
			schedule: LegacyScheduleKey {
				name: schedule.friendly_name,
				period_array: schedule.periods.iter().map(|v| v.clone().into()).collect(),
			},
			theSlot: context[1].clone().map(|v| v.friendly_name),
			time: Local::now().timestamp() as usize,
			theNextSlot: context[2].clone().map(|v| v.friendly_name),
			periodEndTime: context[1].clone().map(|v| v.end.to_string()),
			endOfPreviousPeriod: context[0].clone().map(|v| v.end_timestamp).unwrap_or(0) as usize,
			formattedDate: Local::now().date().to_string(),
			dayOfWeek: Local::now().weekday().to_string(),
			formattedTime: Local::now().time().to_string(),
			scheduleCode: None,     // Unclear what this is for
			isListingForDay: false, // Unclear what this is for
			noSchedule: schedule.periods.len() == 0,
			schoolInSession: schedule
				.periods
				.iter()
				.filter(|v| match v.kind {
					PeriodType::Class(_) => true,
					_ => false,
				})
				.count() > 1,
			school_id: "1".to_string(), // Unclear what this is for
			theNextSlot_: context[2]
				.clone()
				.map(|v| match v.kind {
					PeriodType::Class(n) => n as isize,
					_ => -1,
				})
				.unwrap_or(-1),
			timeLeftInPeriod: context[1]
				.clone()
				.map(|v| (v.end - Local::now().time()).num_seconds())
				.unwrap_or(0) as isize,
			timeSinceLastPeriod: context[0]
				.clone()
				.map(|v| (Local::now().time() - v.end).num_seconds())
				.unwrap_or(0) as isize,
		}
	}
}

impl From<Period> for LegacyPeriod {
	fn from(period: Period) -> Self {
		LegacyPeriod {
			start_time: period.start.to_string(),
			end_time: period.end.to_string(),
			period_notice: Some(format!("{:?}", period.kind)),
			period_name: period.friendly_name,
		}
	}
}
