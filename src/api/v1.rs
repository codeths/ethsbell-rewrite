#![allow(missing_docs)]
#![allow(non_snake_case)]

use super::OurError;
use crate::{
	aliases::v1::NearbyPeriods,
	ical,
	ical::IcalResponder,
	login::Authenticated,
	schedule::{Period, Schedule, ScheduleDefinition, ScheduleType}
};
#[cfg(feature = "ws")]
use crate::schedule::get_schedule_from_config;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
#[cfg(feature = "ws")]
use rocket::{http::Status, response::content::Html, Data, Route, State};
#[cfg(feature = "ws")]
use rocket_contrib::{json::Json, templates::Template};
#[cfg(feature = "ws")]
use rocket_okapi::{openapi, routes_with_openapi};
#[cfg(not(feature = "ws"))]
use crate::api::{Json, State};
use serde::Serialize;
use std::{
	str::FromStr,
	sync::{Arc, RwLock},
};
#[cfg(feature = "ws")]
use std::{
	fs::OpenOptions,
	io::Write,
};

#[cfg(feature = "ws")]
/// Generates a list of Routes for Rocket
pub fn routes() -> Vec<Route> {
	#[allow(unused_mut)]
	let mut r = routes_with_openapi![
		get_schedule,
		today,
		today_code,
		date,
		date_code,
		today_now,
		today_at,
		date_at,
		today_around_now,
		what_time,
		get_spec,
		post_spec,
		post_update,
		check_auth,
		check_version,
		ical,
		coffee,
		widget,
		license,
		schedule_from_to,
	];
	#[cfg(debug_assertions)]
	r.append(&mut routes![force_update]);
	r
}

/// This route is only compiled on debug builds.
/// It forces ETHSBell to rebuild the schedule for testing purposes.
#[cfg(feature = "pull")]
#[cfg(debug_assertions)]
#[cfg_attr(feature = "ws", get("/force-update"))]
pub fn force_update(schedule: State<Arc<RwLock<Schedule>>>) {
	let schedule = schedule.clone();
	schedule.write().unwrap().last_updated = Local::now().naive_local();
	std::thread::spawn(|| Schedule::update_async(schedule));
}

#[derive(Serialize)]
struct WidgetContext {
	prev_name: String,
	current_name: String,
	current_start: String,
	current_end: String,
	next_name: String,
	next_start: String,
	prev_end: String,
}

/// Returns HTML for the output of /today/now/near
/// This is frontend and is not considered in our versioning or tests.
#[cfg(feature = "ws")]
#[cfg_attr(feature = "ws", openapi(skip))]
#[cfg_attr(feature = "ws", get("/widget"))]
fn widget(schedule: State<Arc<RwLock<Schedule>>>) -> Template {
	Schedule::update_if_needed_async(schedule.clone());
	let now = Local::now();
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	let mut schedule = schedule.0.at_time(now_time);
	schedule.0 = schedule.0.map(|v| v.populate(now));
	schedule
		.1
		.iter_mut()
		.for_each(|v| *v = v.clone().populate(now));
	schedule.2 = schedule.2.map(|v| v.populate(now));
	let ctx = WidgetContext {
		prev_name: schedule
			.0
			.clone()
			.map(|v| v.friendly_name)
			.unwrap_or_else(|| "None".to_string()),
		current_name: schedule
			.1
			.clone()
			.iter()
			.map(|v| v.friendly_name.clone())
			.collect::<Vec<String>>()
			.join(", "),
		current_start: schedule
			.1
			.clone()
			.iter()
			.map(|v| v.start.to_string())
			.collect::<Vec<String>>()
			.join(", "),
		current_end: schedule
			.1
			.clone()
			.iter()
			.map(|v| v.end.to_string())
			.collect::<Vec<String>>()
			.join(", "),
		next_name: schedule
			.2
			.clone()
			.map(|v| v.friendly_name)
			.unwrap_or_else(|| "None".to_string()),
		next_start: schedule
			.2
			.clone()
			.map(|v| v.start.to_string())
			.unwrap_or_else(|| "".to_string()),
		prev_end: schedule
			.0
			.clone()
			.map(|v| v.end.to_string())
			.unwrap_or_else(|| "".to_string()),
	};
	Template::render("widget", &ctx)
}

/// Returns a tuple of the crate version, the CI commit hash, and the CI repository.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/check-version"))]
pub fn check_version() -> Json<(String, Option<String>, Option<String>)> {
	Json((
		env!("CARGO_PKG_VERSION").to_string(),
		option_env!("GITHUB_SHA").map(|f| f.to_string()),
		option_env!("GITHUB_REPOSITORY").map(|f| f.to_string()),
	))
}

/// Returns "ok" if the authentication data is valid.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/check-auth"))]
pub fn check_auth(_auth: Authenticated) -> &'static str {
	"ok"
}

/// Fetches the contents of the schedule specification in memory.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/spec"))]
pub fn get_spec(
	schedule: State<Arc<RwLock<Schedule>>>,
) -> Result<Json<ScheduleDefinition>, std::io::Error> {
	Ok(Json(schedule.read().unwrap().definition.clone()))
}

/// Uploads a new schedule specification file.
#[cfg(feature = "ws")]
#[cfg_attr(feature = "ws", openapi(skip))]
#[cfg_attr(feature = "ws", post("/spec", data = "<body>"))]
pub fn post_spec(
	body: Data,
	_auth: Authenticated,
	schedule: State<Arc<RwLock<Schedule>>>,
) -> Result<(), OurError> {
	let mut file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.truncate(true)
		.open("./def.json")?;
	body.stream_to(&mut file)?;
	file.flush()?;
	schedule.write().unwrap().definition = get_schedule_from_config();

	Ok(())
}

/// Update schedule data
#[cfg(feature = "pull")]
#[cfg_attr(feature = "ws", openapi(skip))]
#[cfg_attr(feature = "ws", post("/update"))]
pub fn post_update(
	_auth: Authenticated,
	schedule: State<Arc<RwLock<Schedule>>>,
) -> Result<(), OurError> {
	Schedule::update_async(schedule.clone());
	Ok(())
}

/// Returns the time.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/what-time-is-it?<timestamp>"))]
pub fn what_time(timestamp: Option<i64>) -> String {
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	now.to_rfc2822()
}

/// Returns the entire schedule struct.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/schedule"))]
pub fn get_schedule(schedule: State<Arc<RwLock<Schedule>>>) -> Json<Schedule> {
	Schedule::update_if_needed_async(schedule.clone());
	let schedule = schedule.read().unwrap();
	Json(schedule.clone())
}

/// Returns the schedule type IDs for each day in the range.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/schedule/from/<start>/to/<end>"))]
pub fn schedule_from_to(
	schedule: State<Arc<RwLock<Schedule>>>,
	start: String,
	end: String,
) -> Result<Json<Vec<String>>, OurError> {
	Schedule::update_if_needed_async(schedule.clone());
	let start: NaiveDate = NaiveDate::from_str(&start)?;
	let end: NaiveDate = NaiveDate::from_str(&end)?;
	assert!(start < end);
	let mut cursor = start;
	let mut output = vec![];
	let schedule = schedule.read().unwrap();
	while cursor < end {
		let that_day = schedule.on_date(cursor);
		match that_day.1 {
			Some(v) => output.push(v),
			None => output.push(serde_json::to_string(&that_day.0)?),
		};
		cursor += Duration::days(1);
	}
	Ok(Json(output))
}

/// Returns today's schedule type.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/today?<timestamp>"))]
pub fn today(schedule: State<Arc<RwLock<Schedule>>>, timestamp: Option<i64>) -> Json<ScheduleType> {
	Schedule::update_if_needed_async(schedule.clone());
	// Get the current date as a NaiveDate
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	let now_date = now.date();
	let schedule = schedule.read().unwrap();
	let mut schedule = schedule.on_date(now_date.naive_local());
	schedule
		.0
		.periods
		.iter_mut()
		.for_each(|v| *v = v.clone().populate(now));
	Json(schedule.0)
}

/// Returns today's schedule type ID.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/today/code?<timestamp>"))]
pub fn today_code(
	schedule: State<Arc<RwLock<Schedule>>>,
	timestamp: Option<i64>,
) -> Json<Option<String>> {
	Schedule::update_if_needed_async(schedule.clone());
	// Get the current date as a NaiveDate
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	let now_date = now.date();
	let schedule = schedule.read().unwrap();
	let schedule = schedule.on_date(now_date.naive_local());
	Json(schedule.1)
}

/// Returns the current period type.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/today/now?<timestamp>"))]
pub fn today_now(schedule: State<Arc<RwLock<Schedule>>>, timestamp: Option<i64>) -> Json<Vec<Period>> {
	Schedule::update_if_needed_async(schedule.clone());
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	let mut period = schedule.0.at_time(now_time).1;
	period.iter_mut().for_each(|v| *v = v.clone().populate(now));
	Json(period)
}

/// Returns the current period type and its neighbors.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/today/now/near?<timestamp>"))]
pub fn today_around_now(
	schedule: State<Arc<RwLock<Schedule>>>,
	timestamp: Option<i64>,
) -> Json<NearbyPeriods> {
	Schedule::update_if_needed_async(schedule.clone());
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	let mut schedule = schedule.0.at_time(now_time);
	schedule.0 = schedule.0.map(|v| v.populate(now));
	schedule
		.1
		.iter_mut()
		.for_each(|v| *v = v.clone().populate(now));
	schedule.2 = schedule.2.map(|v| v.populate(now));
	Json(schedule)
}

/// Returns today's period type at the provided time.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/today/at/<time_string>?<timestamp>"))]
pub fn today_at(
	schedule: State<Arc<RwLock<Schedule>>>,
	time_string: String,
	timestamp: Option<i64>,
) -> Result<Option<Json<Vec<Period>>>, OurError> {
	Schedule::update_if_needed_async(schedule.clone());
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	let now_date = now.date();
	let then_time = NaiveTime::from_str(&time_string)?;
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	match schedule.0.at_time(then_time).1 {
		mut period if !period.is_empty() => {
			period.iter_mut().for_each(|v| *v = v.clone().populate(now));
			Ok(Some(Json(period)))
		}
		_ => Ok(None),
	}
}

/// Returns the schedule type on the given date.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/on/<date_string>"))]
pub fn date(
	schedule: State<Arc<RwLock<Schedule>>>,
	date_string: String,
) -> Result<Json<ScheduleType>, OurError> {
	Schedule::update_if_needed_async(schedule.clone());
	let then = NaiveDate::from_str(&date_string)?;
	let then_ = Local::now()
		.with_day(then.day())
		.unwrap()
		.with_month(then.month())
		.unwrap()
		.with_year(then.year())
		.unwrap();
	let mut schedule = schedule.read().unwrap().on_date(then);
	schedule
		.0
		.periods
		.iter_mut()
		.for_each(|v| *v = v.clone().populate(then_));
	Ok(Json(schedule.0))
}

/// Returns the schedule type ID on the given date.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/on/<date_string>/code"))]
pub fn date_code(
	schedule: State<Arc<RwLock<Schedule>>>,
	date_string: String,
) -> Result<Json<Option<String>>, OurError> {
	Schedule::update_if_needed_async(schedule.clone());
	let then = NaiveDate::from_str(&date_string)?;
	let schedule = schedule.read().unwrap().on_date(then);
	Ok(Json(schedule.1))
}

/// Returns the period type at the given date and time.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/on/<date_string>/at/<time_string>"))]
pub fn date_at(
	schedule: State<Arc<RwLock<Schedule>>>,
	date_string: String,
	time_string: String,
) -> Result<Option<Json<Vec<Period>>>, OurError> {
	Schedule::update_if_needed_async(schedule.clone());
	let then_date = NaiveDate::from_str(&date_string)?;
	let then_time = NaiveTime::from_str(&time_string)?;
	let then_ = Local::now()
		.with_day(then_date.day())
		.unwrap()
		.with_month(then_date.month())
		.unwrap()
		.with_year(then_date.year())
		.unwrap();
	let schedule = schedule.read().unwrap().on_date(then_date);
	match schedule.0.at_time(then_time).1 {
		mut period if !period.is_empty() => {
			period
				.iter_mut()
				.for_each(|v| *v = v.clone().populate(then_));
			Ok(Some(Json(period)))
		}
		_ => Ok(None),
	}
}

/// Returns an ICalendar file containing periods as events.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/ical?<backward>&<forward>"))]
pub fn ical(backward: i64, forward: i64, schedule: State<Arc<RwLock<Schedule>>>) -> IcalResponder {
	Schedule::update_if_needed_async(schedule.clone());
	let now = Local::now().date().naive_local();
	let start = now - Duration::days(backward);
	let end = now + Duration::days(forward);
	IcalResponder {
		inner: ical::IcalEvent::generate(&schedule.read().unwrap(), start, end),
	}
}

/// This is an easter egg, but it's also the Docker health check endpoint so don't remove it
#[cfg(feature = "ws")]
#[openapi(skip)]
#[cfg_attr(feature = "ws", get("/coffee"))]
fn coffee(schedule: State<Arc<RwLock<Schedule>>>) -> Status {
	let _lock = schedule.write().unwrap();
	Status::ImATeapot
}

/// Returns the license.
#[cfg(feature = "ws")]
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/license"))]
fn license() -> Html<String> {
	let authors = env!("CARGO_PKG_AUTHORS");
	let authors = authors
		.split(':')
		.map(|v| v.trim())
		.collect::<Vec<&str>>()
		.join(", ");
	Html(format!(
		include_str!("../../LICENSE.html"),
		authors = authors
	))
}
