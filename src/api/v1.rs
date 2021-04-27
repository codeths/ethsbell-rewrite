use std::{
	str::FromStr,
	sync::{Arc, RwLock},
};

use chrono::{Local, NaiveDate, NaiveTime};
use rocket::{Route, State};
use rocket_contrib::json::Json;

use crate::schedule::{Period, Schedule, ScheduleType};

pub fn routes() -> Vec<Route> {
	routes![
		get_schedule,
		today,
		date,
		today_now,
		today_at,
		date_at,
		today_around_now
	]
}

#[get("/schedule")]
fn get_schedule(schedule: State<Arc<RwLock<Schedule>>>) -> Json<Schedule> {
	schedule.write().unwrap().update_if_needed();
	let schedule = schedule.read().unwrap();
	Json(schedule.clone())
}

#[get("/today")]
fn today(schedule: State<Arc<RwLock<Schedule>>>) -> Json<ScheduleType> {
	schedule.write().unwrap().update_if_needed();
	// Get the current date as a NaiveDate
	let now = Local::now();
	println!("{}", now);
	let now = now.date();
	let schedule = schedule.read().unwrap();
	Json(schedule.on_date(now.naive_local()))
}

#[get("/today/now")]
fn today_now(schedule: State<Arc<RwLock<Schedule>>>) -> Option<Json<Period>> {
	schedule.write().unwrap().update_if_needed();
	let now = Local::now();
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	match schedule.at_time(now_time) {
		Some(period) => Some(Json(period)),
		None => None,
	}
}

#[get("/today/now/near")]
fn today_around_now(schedule: State<Arc<RwLock<Schedule>>>) -> Json<[Option<Period>; 3]> {
	schedule.write().unwrap().update_if_needed();
	let now = Local::now();
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	match schedule.at_time(now_time) {
		Some(period) => Json([
			schedule.at_offset(&period, -1),
			Some(period.clone()),
			schedule.at_offset(&period, 1),
		]),
		None => Json([None, None, None]),
	}
}

#[get("/today/at/<time_string>")]
fn today_at(schedule: State<Arc<RwLock<Schedule>>>, time_string: String) -> Option<Json<Period>> {
	schedule.write().unwrap().update_if_needed();
	let now = Local::now();
	let now_date = now.date();
	let then_time = NaiveTime::from_str(&time_string).unwrap();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	match schedule.at_time(then_time) {
		Some(period) => Some(Json(period)),
		None => None,
	}
}

#[get("/on/<date_string>")]
fn date(schedule: State<Arc<RwLock<Schedule>>>, date_string: String) -> Json<ScheduleType> {
	schedule.write().unwrap().update_if_needed();
	let then = NaiveDate::from_str(&date_string).unwrap();
	Json(schedule.read().unwrap().on_date(then))
}

#[get("/on/<date_string>/at/<time_string>")]
fn date_at(
	schedule: State<Arc<RwLock<Schedule>>>,
	date_string: String,
	time_string: String,
) -> Option<Json<Period>> {
	schedule.write().unwrap().update_if_needed();
	let then_date = NaiveDate::from_str(&date_string).unwrap();
	let then_time = NaiveTime::from_str(&time_string).unwrap();
	let schedule = schedule.read().unwrap().on_date(then_date);
	match schedule.at_time(then_time) {
		Some(period) => Some(Json(period)),
		None => None,
	}
}
