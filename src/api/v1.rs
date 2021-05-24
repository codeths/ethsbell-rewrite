use super::OurError;
use crate::{
	login::Authenticated,
	schedule::{Period, Schedule, ScheduleType},
	SpecLock,
};
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use rocket::{Data, Route, State};
use rocket_contrib::json::Json;
use std::{
	fs::{read_to_string, OpenOptions},
	io::Write,
	str::FromStr,
	sync::{Arc, Mutex, RwLock},
};

pub fn routes() -> Vec<Route> {
	routes![
		get_schedule,
		today,
		date,
		today_now,
		today_at,
		date_at,
		today_around_now,
		what_time,
		get_lock,
		force_unlock,
		get_spec,
		post_spec,
	]
}

#[get("/spec")]
fn get_spec(_auth: Authenticated) -> Result<String, std::io::Error> {
	Ok(read_to_string("./def.json")?)
}

#[post("/spec", data = "<body>")]
fn post_spec(body: Data, _auth: Authenticated) -> Result<(), std::io::Error> {
	let mut file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.truncate(true)
		.open("./def.json")?;
	body.stream_to(&mut file)?;
	file.flush()?;
	Ok(())
}

#[get("/lock")]
fn get_lock(
	lock: State<Arc<Mutex<SpecLock>>>,
	_auth: Authenticated,
) -> Result<Json<String>, Json<DateTime<Local>>> {
	let mut lock = lock.lock().unwrap();
	match lock.0 {
		Some(dt) => Err(Json(dt)),
		None => {
			lock.0 = Some(Local::now());
			Ok(Json("OK".to_string()))
		}
	}
}

#[get("/force-unlock")]
fn force_unlock(lock: State<Arc<Mutex<SpecLock>>>, _auth: Authenticated) {
	let mut lock = lock.lock().unwrap();
	lock.0 = None
}

#[get("/what-time-is-it?<timestamp>")]
fn what_time(timestamp: Option<i64>) -> String {
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	now.to_rfc2822()
}

#[get("/schedule")]
fn get_schedule(schedule: State<Arc<RwLock<Schedule>>>) -> Json<Schedule> {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	}
	let schedule = schedule.read().unwrap();
	Json(schedule.clone())
}

#[get("/today?<timestamp>")]
fn today(schedule: State<Arc<RwLock<Schedule>>>, timestamp: Option<i64>) -> Json<ScheduleType> {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	}
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
	schedule.periods.iter_mut().for_each(|v| v.populate(now));
	Json(schedule)
}

#[get("/today/now?<timestamp>")]
fn today_now(
	schedule: State<Arc<RwLock<Schedule>>>,
	timestamp: Option<i64>,
) -> Result<Json<Period>, String> {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	}
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	match schedule.at_time(now_time)[1].clone() {
		Some(period) => {
			let mut period = period.clone();
			period.populate(now);
			Ok(Json(period))
		}
		None => Err(String::from("There's no schedule right now, sorry.")),
	}
}

#[get("/today/now/near?<timestamp>")]
fn today_around_now(
	schedule: State<Arc<RwLock<Schedule>>>,
	timestamp: Option<i64>,
) -> Json<[Option<Period>; 3]> {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	}
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	let mut schedule = schedule.at_time(now_time);
	schedule.iter_mut().for_each(|v| match v {
		Some(v) => v.populate(now),
		None => {}
	});
	Json(schedule)
}

#[get("/today/at/<time_string>?<timestamp>")]
fn today_at(
	schedule: State<Arc<RwLock<Schedule>>>,
	time_string: String,
	timestamp: Option<i64>,
) -> Result<Option<Json<Period>>, OurError> {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	}
	let now = match timestamp {
		None => Local::now(),
		Some(timestamp) => Local
			.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
			.with_timezone(&Local),
	};
	let now_date = now.date();
	let then_time = NaiveTime::from_str(&time_string)?;
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	match schedule.at_time(then_time)[1].clone() {
		Some(period) => {
			let mut period = period.clone();
			period.populate(now);
			Ok(Some(Json(period)))
		}
		None => Ok(None),
	}
}

#[get("/on/<date_string>")]
fn date(
	schedule: State<Arc<RwLock<Schedule>>>,
	date_string: String,
) -> Result<Json<ScheduleType>, OurError> {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	}
	let then = NaiveDate::from_str(&date_string)?;
	let then_ = Local::now()
		.with_day(then.day())
		.unwrap()
		.with_month(then.month())
		.unwrap()
		.with_year(then.year())
		.unwrap();
	let mut schedule = schedule.read().unwrap().on_date(then);
	schedule.periods.iter_mut().for_each(|v| v.populate(then_));
	Ok(Json(schedule))
}

#[get("/on/<date_string>/at/<time_string>")]
fn date_at(
	schedule: State<Arc<RwLock<Schedule>>>,
	date_string: String,
	time_string: String,
) -> Result<Option<Json<Period>>, OurError> {
	if schedule.read().unwrap().is_update_needed() {
		schedule.write().unwrap().update();
	}
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
	match schedule.at_time(then_time)[1].clone() {
		Some(period) => {
			let mut period = period.clone();
			period.populate(then_);
			Ok(Some(Json(period)))
		}
		None => Ok(None),
	}
}
