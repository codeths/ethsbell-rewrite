#![cfg(feature = "ws")]
#![allow(clippy::too_many_lines)]

use chrono::{Local, NaiveDateTime};
use ethsbell_rewrite::{
	aliases::v2::NearbyPeriods,
	rocket_builder::rocket,
	schedule::{Period, PeriodType, Schedule, ScheduleType},
};
use rocket::{
	async_test,
	http::{ContentType, Status},
	local::asynchronous::Client,
};

async fn client(schedule: Schedule) -> Client {
	Client::tracked(rocket(schedule)).await.unwrap()
}

#[async_test]
async fn now_near() {
	let mut schedule = Schedule::default();
	// Add test A
	let period_now = Period {
		friendly_name: "Block 2".to_string(),
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
	let period_before_1 = Period {
		friendly_name: "Block 1A".to_string(),
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
	let mut period_before_2 = period_before_1.clone();
	period_before_2.friendly_name = "Block 1B".to_string();
	let period_after = Period {
		friendly_name: "Block 3".to_string(),
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
			period_before_1.clone(),
			period_before_2.clone(),
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
	let response = client.get("/api/v2/today/now/near").dispatch().await;
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	let json_string = &response.into_string().await.unwrap();
	println!("API response: {json_string}");
	let response: NearbyPeriods = serde_json::from_str(json_string).unwrap();
	println!("Deserialized: {response:?}");

	assert_eq!(
		response,
		NearbyPeriods {
			previous: vec![period_before_1, period_before_2],
			current: vec![period_now],
			future: vec![period_after]
		}
	);
}
