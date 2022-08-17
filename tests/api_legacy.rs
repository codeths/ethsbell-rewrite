#![cfg(feature = "ws")]

use chrono::NaiveTime;
use ethsbell_rewrite::{
	rocket_builder::rocket,
	schedule::{Period, PeriodType, Schedule, ScheduleType},
};
use rocket::{http::Status, local::Client};
// This file is mostly just here to make sure the legacy endpoints don't panic; it's up to you to keep them working correctly.

fn client(schedule: Schedule) -> Client {
	Client::new(rocket(schedule)).unwrap()
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
		hide: false,
		friendly_name: "Test A".to_string(),
		periods: vec![period],
		regex: None,
		color: Some([0, 0, 0]),
	};
	schedule
		.definition
		.schedule_types
		.insert("test_a".to_string(), test_a);
	// Build typical schedule
	schedule.definition.typical_schedule = vec!["test_a".to_string(); 7];
	let client = client(schedule.clone());
	// Check test A
	let response = client.get("/api/legacy/display").dispatch();
	assert_eq!(response.status(), Status::Ok);
	let response = client.get("/api/legacy/data").dispatch();
	assert_eq!(response.status(), Status::Ok);
}
