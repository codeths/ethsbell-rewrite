#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(clippy::let_unit_value)]

#[cfg(not(feature = "ws"))]
use crate::api::{Json, State};
use crate::{aliases::v2::NearbyPeriods, schedule::Schedule};
use chrono::{Local, NaiveDateTime, TimeZone};
#[cfg(feature = "ws")]
use rocket::serde::json::Json;
#[cfg(feature = "ws")]
use rocket::{Route, State};
#[cfg(feature = "ws")]
use rocket_okapi::openapi;
use std::sync::{Arc, RwLock};

#[cfg(feature = "ws")]
#[must_use]
/// Generates a list of Routes for Rocket
pub fn routes() -> Vec<Route> {
	use rocket_okapi::settings::OpenApiSettings;
	let settings = OpenApiSettings::new();
	let spec = rocket_okapi::openapi_spec![today_around_now](&settings);
	#[allow(unused_mut)]
	let mut r = rocket_okapi::openapi_routes![today_around_now](Some(spec), &settings);
	r
}

/// Returns the current period type and its neighbors.
#[cfg_attr(feature = "ws", openapi)]
#[cfg_attr(feature = "ws", get("/today/now/near?<timestamp>"))]
#[must_use]
pub fn today_around_now(
	schedule: &State<Arc<RwLock<Schedule>>>,
	timestamp: Option<i64>,
) -> Json<NearbyPeriods> {
	Schedule::update_if_needed_async(schedule.inner().clone());
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default())
			.with_timezone(&Local),
	};
	let now_date = now.date_naive();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date);
	let mut schedule = schedule.0.at_time_v2(now_time);
	schedule
		.0
		.iter_mut()
		.for_each(|v| *v = v.clone().populate(now));
	schedule
		.1
		.iter_mut()
		.for_each(|v| *v = v.clone().populate(now));
	schedule
		.2
		.iter_mut()
		.for_each(|v| *v = v.clone().populate(now));

	Json(NearbyPeriods {
		previous: schedule.0,
		current: schedule.1,
		future: schedule.2,
	})
}
