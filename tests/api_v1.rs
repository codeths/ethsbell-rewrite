#![cfg(feature = "ws")]
#![allow(clippy::too_many_lines)]

use chrono::{Local, NaiveDateTime, NaiveTime};
use ethsbell_rewrite::{
	rocket_builder::rocket,
	schedule::{Period, PeriodType, Schedule, ScheduleType},
};
use regex::Regex;
use rocket::{
	async_test,
	http::{ContentType, Status},
	local::asynchronous::Client,
};

async fn client(schedule: Schedule) -> Client {
	Client::tracked(rocket(schedule)).await.unwrap()
}

#[async_test]
async fn schedule() {
	let schedule = Schedule {
		last_updated: Local::now().naive_local(),
		..Schedule::default()
	};
	let client = client(schedule.clone()).await;
	let response = client.get("/api/v1/schedule").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.into_string().await.unwrap(),
		serde_json::to_string(&schedule).unwrap()
	);
}

#[async_test]
async fn spec() {
	let schedule = Schedule {
		last_updated: Local::now().naive_local(),
		..Schedule::default()
	};
	let client = client(schedule.clone()).await;
	let response = client.get("/api/v1/spec").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.into_string().await.unwrap(),
		serde_json::to_string(&schedule.definition).unwrap()
	);
}

#[async_test]
async fn on() {
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
			.map(|v| (*v).to_string())
			.collect();
	let client = client(schedule.clone()).await;
	// Check test A
	let response = client.get("/api/v1/on/2021-07-27").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.into_string().await.unwrap(),
		serde_json::to_string(&test_a).unwrap()
	);
	// Check "no"
	let response = client.get("/api/v1/on/2021-07-26").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.into_string().await.unwrap(),
		serde_json::to_string(&no).unwrap()
	);
}

#[async_test]
async fn on_code() {
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
			.map(std::string::ToString::to_string)
			.collect();
	let client = client(schedule.clone()).await;
	// Check test A
	let response = client.get("/api/v1/on/2021-07-27/code").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.into_string().await.unwrap(),
		serde_json::to_string("test_a").unwrap()
	);
	// Check "no"
	let response = client.get("/api/v1/on/2021-07-26/code").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.into_string().await.unwrap(),
		serde_json::to_string("no").unwrap()
	);
}

#[async_test]
async fn on_at() {
	let mut schedule = Schedule::default();
	// Add test A
	let period = Period {
		friendly_name: String::new(),
		start: NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default(),
		start_timestamp: 0,
		end: NaiveTime::from_hms_opt(10, 0, 0).unwrap_or_default(),
		end_timestamp: 0,
		kind: PeriodType::Class(String::new()),
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
	let client = client(schedule.clone()).await;
	// Check test A
	let response = client
		.get("/api/v1/on/2021-07-27/at/09:26:00")
		.dispatch()
		.await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let mut response: Vec<Period> =
		serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
	response[0].start_timestamp = 0;
	response[0].end_timestamp = 0;
	assert_eq!(response, vec![period]);
}

#[async_test]
async fn now() {
	let mut schedule = Schedule::default();
	// Add test A
	let period = Period {
		friendly_name: String::new(),
		start: {
			let now = Local::now().naive_local().timestamp() - 10;
			NaiveDateTime::from_timestamp_opt(now, 0)
				.unwrap_or_default()
				.time()
		},
		start_timestamp: 0,
		end: {
			let now = Local::now().naive_local().timestamp() + 10;
			NaiveDateTime::from_timestamp_opt(now, 0)
				.unwrap_or_default()
				.time()
		},
		end_timestamp: 0,
		kind: PeriodType::Class(String::new()),
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
	let client = client(schedule.clone()).await;
	// Check test A
	let response = client.get("/api/v1/today/now").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let mut response: Vec<Period> =
		serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
	response[0].start_timestamp = 0;
	response[0].end_timestamp = 0;
	assert_eq!(response, vec![period]);
}

#[async_test]
async fn now_near() {
	let mut schedule = Schedule::default();
	// Add test A
	let period_now = Period {
		friendly_name: String::new(),
		start: {
			let now = Local::now().naive_local().timestamp() - 10;
			NaiveDateTime::from_timestamp_opt(now, 0)
				.unwrap_or_default()
				.time()
		},
		start_timestamp: 0,
		end: {
			let now = Local::now().naive_local().timestamp() + 10;
			NaiveDateTime::from_timestamp_opt(now, 0)
				.unwrap_or_default()
				.time()
		},
		end_timestamp: 0,
		kind: PeriodType::Class(String::new()),
	};
	let period_before = Period {
		friendly_name: String::new(),
		start: {
			let now = Local::now().naive_local().timestamp() - 30;
			NaiveDateTime::from_timestamp_opt(now, 0)
				.unwrap_or_default()
				.time()
		},
		start_timestamp: 0,
		end: {
			let now = Local::now().naive_local().timestamp() - 20;
			NaiveDateTime::from_timestamp_opt(now, 0)
				.unwrap_or_default()
				.time()
		},
		end_timestamp: 0,
		kind: PeriodType::Class(String::new()),
	};
	let period_after = Period {
		friendly_name: String::new(),
		start: {
			let now = Local::now().naive_local().timestamp() + 20;
			NaiveDateTime::from_timestamp_opt(now, 0)
				.unwrap_or_default()
				.time()
		},
		start_timestamp: 0,
		end: {
			let now = Local::now().naive_local().timestamp() + 30;
			NaiveDateTime::from_timestamp_opt(now, 0)
				.unwrap_or_default()
				.time()
		},
		end_timestamp: 0,
		kind: PeriodType::Class(String::new()),
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
	let client = client(schedule.clone()).await;
	// Check test A
	let response = client.get("/api/v1/today/now/near").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let mut response: (Option<Period>, Vec<Period>, Option<Period>) =
		serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
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

#[async_test]
async fn today_at() {
	let mut schedule = Schedule::default();
	// Add test A
	let period = Period {
		friendly_name: String::new(),
		start: NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default(),
		start_timestamp: 0,
		end: NaiveTime::from_hms_opt(10, 0, 0).unwrap_or_default(),
		end_timestamp: 0,
		kind: PeriodType::Class(String::new()),
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
	let client = client(schedule.clone()).await;
	// Check test A
	let response = client.get("/api/v1/today/at/09:30:00").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let mut response: Vec<Period> =
		serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
	response[0].start_timestamp = 0;
	response[0].end_timestamp = 0;
	assert_eq!(response, vec![period]);
}

#[async_test]
async fn today() {
	let mut schedule = Schedule::default();
	// Add test A
	let period = Period {
		friendly_name: String::new(),
		start: NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default(),
		start_timestamp: 0,
		end: NaiveTime::from_hms_opt(10, 0, 0).unwrap_or_default(),
		end_timestamp: 0,
		kind: PeriodType::Class(String::new()),
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
	let client = client(schedule.clone()).await;
	// Check test A
	let response = client.get("/api/v1/today").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let response: ScheduleType =
		serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
	assert_eq!(response, test_a);
}

#[async_test]
async fn today_code() {
	let mut schedule = Schedule::default();
	// Add test A
	let period = Period {
		friendly_name: String::new(),
		start: NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default(),
		start_timestamp: 0,
		end: NaiveTime::from_hms_opt(10, 0, 0).unwrap_or_default(),
		end_timestamp: 0,
		kind: PeriodType::Class(String::new()),
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
	let client = client(schedule.clone()).await;
	// Check test A
	let response = client.get("/api/v1/today/code").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(response.into_string().await.unwrap(), "\"test_a\"");
}
