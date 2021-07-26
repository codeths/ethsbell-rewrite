#![cfg(feature = "ws")]

use std::sync::{Arc, Mutex, RwLock};

use chrono::NaiveTime;
use ethsbell_rewrite::{
	schedule::{Period, PeriodType, Schedule, ScheduleType},
	SpecLock,
};
use rocket::{http::Status, local::Client};
use rocket_contrib::templates::Template;
// This file is mostly just here to make sure the legacy endpoints don't panic; it's up to you to keep them working correctly.

fn client(schedule: Schedule) -> Client {
	let schedule = Arc::new(RwLock::new(schedule));
	let spec_lock = Arc::new(Mutex::new(SpecLock(None)));
	Client::new(
		rocket::ignite()
			.attach(ethsbell_rewrite::api::ApiFairing)
			.attach(ethsbell_rewrite::frontend::FrontendFairing)
			.attach(Template::fairing())
			.manage(schedule.clone())
			.manage(spec_lock),
	)
	.unwrap()
}

#[test]
fn things() {
	let mut schedule = Schedule::default();
	// Add test A
	let period = Period {
		friendly_name: "".to_string(),
		start: NaiveTime::from_hms(9, 0, 0),
		start_timestamp: 0,
		end: NaiveTime::from_hms(10, 0, 0),
		end_timestamp: 0,
		kind: PeriodType::Class("".to_string()),
	};
	let test_a = ScheduleType {
		friendly_name: "Test A".to_string(),
		periods: vec![period.clone()],
		regex: None,
		color: Some([0, 0, 0]),
	};
	schedule
		.definition
		.schedule_types
		.insert("test_a".to_string(), test_a.clone());
	// Build typical schedule
	schedule.definition.typical_schedule = vec!["test_a".to_string(); 7];
	let client = client(schedule.clone());
	// Check test A
	let response = client.get("/api/legacy/display").dispatch();
	assert_eq!(response.status(), Status::Ok);
	let response = client.get("/api/legacy/data").dispatch();
	assert_eq!(response.status(), Status::Ok);
}
