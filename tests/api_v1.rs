#![cfg(feature = "ws")]

use chrono::{Local, NaiveDateTime, NaiveTime};
use ethsbell_rewrite::{
	rocket_builder::rocket,
	schedule::{Period, PeriodType, Schedule, ScheduleType},
};
use regex::Regex;
use rocket::{
	http::{ContentType, Status},
	local::Client,
};

fn client(schedule: Schedule) -> Client {
	Client::new(rocket(schedule)).unwrap()
}

#[test]
fn schedule() {
	let mut schedule = Schedule::default();
	schedule.last_updated = Local::now().naive_local();
	let client = client(schedule.clone());
	let mut response = client.get("/api/v1/schedule").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.body_string(),
		Some(serde_json::to_string(&schedule).unwrap())
	);
}

#[test]
fn spec() {
	let mut schedule = Schedule::default();
	schedule.last_updated = Local::now().naive_local();
	let client = client(schedule.clone());
	let mut response = client.get("/api/v1/spec").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.body_string(),
		Some(serde_json::to_string(&schedule.definition).unwrap())
	);
}

#[test]
fn on() {
	let mut schedule = Schedule::default();
	// Add test A
	let test_a = ScheduleType {
		hide: false,
		friendly_name: "Test A".to_string(),
		periods: vec![],
		regex: None,
		color: Some([0, 0, 0]),
	};
	schedule
		.definition
		.schedule_types
		.insert("test_a".to_string(), test_a.clone());
	// Add no
	let no = ScheduleType {
		hide: false,
		friendly_name: "No".to_string(),
		periods: vec![],
		regex: Some(Regex::new("No").unwrap()),
		color: Some([0, 0, 0]),
	};
	schedule
		.definition
		.schedule_types
		.insert("no".to_string(), no.clone());
	// Build typical schedule
	schedule.definition.typical_schedule =
		vec!["test_a", "no", "test_a", "no", "test_a", "no", "test_a"]
			.iter()
			.map(|v| v.to_string())
			.collect();
	let client = client(schedule.clone());
	// Check test A
	let mut response = client.get("/api/v1/on/2021-07-27").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.body_string(),
		Some(serde_json::to_string(&test_a).unwrap())
	);
	// Check "no"
	let mut response = client.get("/api/v1/on/2021-07-26").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.body_string(),
		Some(serde_json::to_string(&no).unwrap())
	)
}

#[test]
fn on_code() {
	let mut schedule = Schedule::default();
	// Add test A
	let test_a = ScheduleType {
		hide: false,
		friendly_name: "Test A".to_string(),
		periods: vec![],
		regex: None,
		color: Some([0, 0, 0]),
	};
	schedule
		.definition
		.schedule_types
		.insert("test_a".to_string(), test_a);
	// Add no
	let no = ScheduleType {
		hide: false,
		friendly_name: "No".to_string(),
		periods: vec![],
		regex: Some(Regex::new("No").unwrap()),
		color: Some([0, 0, 0]),
	};
	schedule
		.definition
		.schedule_types
		.insert("no".to_string(), no);
	// Build typical schedule
	schedule.definition.typical_schedule =
		vec!["test_a", "no", "test_a", "no", "test_a", "no", "test_a"]
			.iter()
			.map(|v| v.to_string())
			.collect();
	let client = client(schedule.clone());
	// Check test A
	let mut response = client.get("/api/v1/on/2021-07-27/code").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.body_string(),
		Some(serde_json::to_string("test_a").unwrap())
	);
	// Check "no"
	let mut response = client.get("/api/v1/on/2021-07-26/code").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.body_string(),
		Some(serde_json::to_string("no").unwrap())
	)
}

#[test]
fn on_at() {
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
		periods: vec![period.clone()],
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
	let mut response = client.get("/api/v1/on/2021-07-27/at/09:26:00").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let mut response: Vec<Period> = serde_json::from_str(&response.body_string().unwrap()).unwrap();
	response[0].start_timestamp = 0;
	response[0].end_timestamp = 0;
	assert_eq!(response, vec![period]);
}

#[test]
fn now() {
	let mut schedule = Schedule::default();
	// Add test A
	let period = Period {
		friendly_name: "".to_string(),
		start: {
			let now = Local::now().naive_local().timestamp() - 10;
			NaiveDateTime::from_timestamp(now, 0).time()
		},
		start_timestamp: 0,
		end: {
			let now = Local::now().naive_local().timestamp() + 10;
			NaiveDateTime::from_timestamp(now, 0).time()
		},
		end_timestamp: 0,
		kind: PeriodType::Class("".to_string()),
	};
	let test_a = ScheduleType {
		hide: false,
		friendly_name: "Test A".to_string(),
		periods: vec![period.clone()],
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
	let mut response = client.get("/api/v1/today/now").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let mut response: Vec<Period> = serde_json::from_str(&response.body_string().unwrap()).unwrap();
	response[0].start_timestamp = 0;
	response[0].end_timestamp = 0;
	assert_eq!(response, vec![period]);
}

#[test]
fn now_near() {
	let mut schedule = Schedule::default();
	// Add test A
	let period_now = Period {
		friendly_name: "".to_string(),
		start: {
			let now = Local::now().naive_local().timestamp() - 10;
			NaiveDateTime::from_timestamp(now, 0).time()
		},
		start_timestamp: 0,
		end: {
			let now = Local::now().naive_local().timestamp() + 10;
			NaiveDateTime::from_timestamp(now, 0).time()
		},
		end_timestamp: 0,
		kind: PeriodType::Class("".to_string()),
	};
	let period_before = Period {
		friendly_name: "".to_string(),
		start: {
			let now = Local::now().naive_local().timestamp() - 30;
			NaiveDateTime::from_timestamp(now, 0).time()
		},
		start_timestamp: 0,
		end: {
			let now = Local::now().naive_local().timestamp() - 20;
			NaiveDateTime::from_timestamp(now, 0).time()
		},
		end_timestamp: 0,
		kind: PeriodType::Class("".to_string()),
	};
	let period_after = Period {
		friendly_name: "".to_string(),
		start: {
			let now = Local::now().naive_local().timestamp() + 20;
			NaiveDateTime::from_timestamp(now, 0).time()
		},
		start_timestamp: 0,
		end: {
			let now = Local::now().naive_local().timestamp() + 30;
			NaiveDateTime::from_timestamp(now, 0).time()
		},
		end_timestamp: 0,
		kind: PeriodType::Class("".to_string()),
	};
	let test_a = ScheduleType {
		hide: false,
		friendly_name: "Test A".to_string(),
		periods: vec![
			period_before.clone(),
			period_now.clone(),
			period_after.clone(),
		],
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
	let mut response = client.get("/api/v1/today/now/near").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let mut response: (Option<Period>, Vec<Period>, Option<Period>) =
		serde_json::from_str(&response.body_string().unwrap()).unwrap();
	response.1[0].start_timestamp = 0;
	response.1[0].end_timestamp = 0;
	// this is
	response.0 = response.0.map(|v| {
		let mut p = v;
		p.start_timestamp = 0;
		p
	});
	response.0 = response.0.map(|v| {
		let mut p = v;
		p.end_timestamp = 0;
		p
	});
	response.2 = response.2.map(|v| {
		let mut p = v;
		p.start_timestamp = 0;
		p
	});
	response.2 = response.2.map(|v| {
		let mut p = v;
		p.end_timestamp = 0;
		p
	});
	assert_eq!(
		response,
		(Some(period_before), vec![period_now], Some(period_after))
	);
}

#[test]
fn today_at() {
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
		periods: vec![period.clone()],
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
	let mut response = client.get("/api/v1/today/at/09:30:00").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let mut response: Vec<Period> = serde_json::from_str(&response.body_string().unwrap()).unwrap();
	response[0].start_timestamp = 0;
	response[0].end_timestamp = 0;
	assert_eq!(response, vec![period]);
}

#[test]
fn today() {
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
		.insert("test_a".to_string(), test_a.clone());
	// Build typical schedule
	schedule.definition.typical_schedule = vec!["test_a".to_string(); 7];
	let client = client(schedule.clone());
	// Check test A
	let mut response = client.get("/api/v1/today").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let response: ScheduleType = serde_json::from_str(&response.body_string().unwrap()).unwrap();
	assert_eq!(response, test_a);
}

#[test]
fn today_code() {
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
	let mut response = client.get("/api/v1/today/code").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(response.body_string().unwrap(), "\"test_a\"");
}
