use std::collections::HashMap;

use chrono::{Local, NaiveDate, NaiveTime};
use ethsbell_rewrite::ical::IcalEvent;
use ethsbell_rewrite::schedule::{
	Event, Period, PeriodType, Schedule, ScheduleDefinition, ScheduleType,
};

#[test]
fn on_date_typical() {
	let mut schedule = Schedule::default();
	let type_a = ScheduleType {
		hide: false,
		color: None,
		friendly_name: "Test A".to_string(),
		periods: vec![],
		regex: None,
	};
	let type_b = ScheduleType {
		hide: false,
		color: None,
		friendly_name: "Test A".to_string(),
		periods: vec![],
		regex: None,
	};
	schedule.last_updated = Local::now().naive_local();
	schedule.definition.schedule_types = {
		let mut result = HashMap::new();
		result.insert("type_a".to_string(), type_a.clone());
		result.insert("type_b".to_string(), type_b.clone());
		result
	};
	schedule.definition.typical_schedule = vec![
		"type_a".to_string(),
		"type_b".to_string(),
		"type_a".to_string(),
		"type_b".to_string(),
		"type_a".to_string(),
		"type_a".to_string(),
		"type_b".to_string(),
	];
	assert_eq!(
		schedule.on_date(NaiveDate::from_ymd_opt(2021, 7, 21).unwrap_or_default()),
		(type_b, Some("type_b".to_string()))
	);

	assert_eq!(
		schedule.on_date(NaiveDate::from_ymd_opt(2021, 7, 18).unwrap_or_default()),
		(type_a, Some("type_a".to_string()))
	);
}

#[test]
fn on_date_override() {
	let date = NaiveDate::from_ymd_opt(2021, 7, 20).unwrap_or_default();
	let type_a = ScheduleType {
		hide: false,
		color: None,
		friendly_name: "Test A".to_string(),
		periods: vec![],
		regex: None,
	};
	let type_b = ScheduleType {
		hide: false,
		color: None,
		friendly_name: "Test A".to_string(),
		periods: vec![],
		regex: None,
	};
	let schedule = Schedule {
		last_updated: Local::now().naive_local(),
		calendar: {
			let mut result = HashMap::new();
			result.insert(date, vec![Event::ScheduleOverride("type_b".to_string())]);
			result
		},
		definition: ScheduleDefinition {
			calendar_urls: vec![],
			schedule_types: {
				let mut result = HashMap::new();
				result.insert("type_a".to_string(), type_a);
				result.insert("type_b".to_string(), type_b.clone());
				result
			},
			typical_schedule: vec![
				"type_a".to_string(),
				"type_b".to_string(),
				"type_a".to_string(),
				"type_b".to_string(),
				"type_a".to_string(),
				"type_a".to_string(),
				"type_b".to_string(),
			],
		},
	};
	assert_eq!(
		schedule.on_date(NaiveDate::from_ymd_opt(2021, 7, 20).unwrap_or_default()),
		(type_b, Some("type_b".to_string()))
	);
}

#[test]
fn on_date_literal() {
	let literal = ScheduleType {
		hide: false,
		color: None,
		friendly_name: "test_c".to_string(),
		periods: vec![],
		regex: None,
	};
	let date = NaiveDate::from_ymd_opt(2021, 7, 20).unwrap_or_default();
	let schedule = Schedule {
		last_updated: Local::now().naive_local(),
		calendar: {
			let mut result = HashMap::new();
			result.insert(
				date,
				vec![Event::ScheduleLiteral(
					serde_json::to_string(&literal).unwrap(),
				)],
			);
			result
		},
		definition: ScheduleDefinition {
			calendar_urls: vec![],
			schedule_types: HashMap::new(),
			typical_schedule: vec![],
		},
	};
	assert_eq!(schedule.on_date(date), (literal, None));
}

#[test]
fn at_time_typical() {
	let mut test_period = Period {
		friendly_name: "test_period".to_string(),
		start: NaiveTime::from_hms_opt(8, 0, 0).unwrap_or_default(),
		start_timestamp: 0,
		end: NaiveTime::from_hms_opt(16, 0, 0).unwrap_or_default(),
		end_timestamp: 0,
		kind: PeriodType::Lunch,
	};
	let schedule = ScheduleType {
		hide: false,
		color: None,
		friendly_name: String::new(),
		periods: vec![test_period.clone()],
		regex: None,
	};
	// timestamps won't be the same, that's fine
	let new = schedule
		.at_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap_or_default())
		.1;
	test_period.start_timestamp = new[0].start_timestamp;
	test_period.end_timestamp = new[0].end_timestamp;
	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap_or_default())
			.1,
		vec![test_period.clone()]
	);

	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(17, 0, 0).unwrap_or_default())
			.0,
		Some(test_period.clone())
	);

	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(6, 0, 0).unwrap_or_default())
			.2,
		Some(test_period)
	);
}

#[test]
fn at_time_pseudo() {
	let schedule = ScheduleType {
		hide: false,
		color: None,
		friendly_name: String::new(),
		periods: vec![
			Period {
				friendly_name: "test_period_a".to_string(),
				start: NaiveTime::from_hms_opt(8, 0, 0).unwrap_or_default(),
				start_timestamp: 0,
				end: NaiveTime::from_hms_opt(12, 0, 0).unwrap_or_default(),
				end_timestamp: 0,
				kind: PeriodType::Lunch,
			},
			Period {
				friendly_name: "test_period_b".to_string(),
				start: NaiveTime::from_hms_opt(14, 0, 0).unwrap_or_default(),
				start_timestamp: 0,
				end: NaiveTime::from_hms_opt(16, 0, 0).unwrap_or_default(),
				end_timestamp: 0,
				kind: PeriodType::Lunch,
			},
		],
		regex: None,
	};
	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(13, 0, 0).unwrap_or_default())
			.1[0]
			.friendly_name,
		"Passing Period"
	);

	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(17, 0, 0).unwrap_or_default())
			.1[0]
			.friendly_name,
		"After School"
	);

	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(6, 0, 0).unwrap_or_default())
			.1[0]
			.friendly_name,
		"Before School"
	);
}

#[test]
fn at_time_overlap() {
	let schedule = ScheduleType {
		hide: false,
		color: None,
		friendly_name: String::new(),
		periods: vec![
			Period {
				friendly_name: "test_period_a".to_string(),
				start: NaiveTime::from_hms_opt(8, 0, 0).unwrap_or_default(),
				start_timestamp: 0,
				end: NaiveTime::from_hms_opt(14, 0, 0).unwrap_or_default(),
				end_timestamp: 0,
				kind: PeriodType::Lunch,
			},
			Period {
				friendly_name: "test_period_b".to_string(),
				start: NaiveTime::from_hms_opt(10, 0, 0).unwrap_or_default(),
				start_timestamp: 0,
				end: NaiveTime::from_hms_opt(16, 0, 0).unwrap_or_default(),
				end_timestamp: 0,
				kind: PeriodType::Lunch,
			},
		],
		regex: None,
	};
	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default())
			.1[0]
			.friendly_name,
		"test_period_a"
	);

	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(15, 0, 0).unwrap_or_default())
			.1[0]
			.friendly_name,
		"test_period_b"
	);

	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap_or_default())
			.1
			.iter()
			.map(|v| v.friendly_name.clone())
			.collect::<Vec<String>>(),
		vec!["test_period_a", "test_period_b"]
	);
}

#[test]
fn at_time_envelop() {
	let schedule = ScheduleType {
		hide: false,
		color: None,
		friendly_name: String::new(),
		periods: vec![
			Period {
				friendly_name: "test_period_a".to_string(),
				start: NaiveTime::from_hms_opt(8, 0, 0).unwrap_or_default(),
				start_timestamp: 0,
				end: NaiveTime::from_hms_opt(16, 0, 0).unwrap_or_default(),
				end_timestamp: 0,
				kind: PeriodType::Lunch,
			},
			Period {
				friendly_name: "test_period_b".to_string(),
				start: NaiveTime::from_hms_opt(10, 0, 0).unwrap_or_default(),
				start_timestamp: 0,
				end: NaiveTime::from_hms_opt(12, 0, 0).unwrap_or_default(),
				end_timestamp: 0,
				kind: PeriodType::Lunch,
			},
		],
		regex: None,
	};
	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default())
			.1[0]
			.friendly_name,
		"test_period_a"
	);

	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(17, 0, 0).unwrap_or_default())
			.1[0]
			.friendly_name,
		"After School"
	);

	assert_eq!(
		schedule
			.at_time(NaiveTime::from_hms_opt(11, 0, 0).unwrap_or_default())
			.1
			.iter()
			.map(|v| v.friendly_name.clone())
			.collect::<Vec<String>>(),
		vec!["test_period_a", "test_period_b"]
	);
}

#[test]
fn at_time_no_schedule() {
	let schedule = ScheduleType {
		hide: false,
		color: None,
		friendly_name: String::new(),
		periods: vec![],
		regex: None,
	};
	assert_eq!(
		schedule.at_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap_or_default()),
		(None, vec![], None)
	);
}

#[cfg(feature = "pull")]
#[test]
fn schedule_pull() {
	let mut definition = Schedule::default().definition;
	definition.calendar_urls =
		vec!["https://www.eths.k12.il.us/site/handlers/icalfeed.ashx?MIID=1".to_string()];
	let _schedule = Schedule::from(definition);
}

#[cfg(feature = "pull")]
#[test]
fn schedule_ical() {
	use chrono::Duration;
	use regex::Regex;

	let mut definition = Schedule::default().definition;
	definition.schedule_types.insert(
		"no".to_string(),
		ScheduleType {
			hide: false,
			friendly_name: "No".to_string(),
			periods: vec![Period {
				friendly_name: "No".to_string(),
				start: Local::now().naive_local().time(),
				end: Local::now().naive_local().time() + Duration::hours(2),
				kind: PeriodType::Break,
				start_timestamp: 0,
				end_timestamp: 0,
			}],
			regex: Some(Regex::new("[Nn]o").unwrap()),
			color: Some([0; 3]),
		},
	);
	definition.schedule_types.insert(
		"yes".to_string(),
		ScheduleType {
			hide: false,
			friendly_name: "Yes".to_string(),
			periods: vec![Period {
				friendly_name: "Yes".to_string(),
				start: Local::now().naive_local().time(),
				end: Local::now().naive_local().time() + Duration::hours(1),
				kind: PeriodType::Break,
				start_timestamp: 0,
				end_timestamp: 0,
			}],
			regex: None,
			color: Some([0; 3]),
		},
	);
	definition.typical_schedule = vec!["yes".to_string(); 7];
	definition.calendar_urls =
		vec!["https://www.eths.k12.il.us/site/handlers/icalfeed.ashx?MIID=1".to_string()];
	let schedule = Schedule::from(definition);
	let _ical = IcalEvent::generate(
		&schedule,
		NaiveDate::from_ymd_opt(2020, 1, 1).unwrap_or_default(),
		NaiveDate::from_ymd_opt(2022, 1, 1).unwrap_or_default(),
	);
}

#[cfg(feature = "pull")]
#[test]
fn schedule_generate() {
	use std::str::FromStr;

	use regex::Regex;

	let mut schedule = Schedule::default();
	schedule.definition.schedule_types = {
		let mut result = HashMap::new();
		result.insert(
			"type_a".to_string(),
			ScheduleType {
				hide: false,
				color: None,
				friendly_name: "Test".to_string(),
				periods: vec![
					Period {
						friendly_name: "test_period_a".to_string(),
						start: NaiveTime::from_hms_opt(8, 0, 0).unwrap_or_default(),
						start_timestamp: 0,
						end: NaiveTime::from_hms_opt(12, 0, 0).unwrap_or_default(),
						end_timestamp: 0,
						kind: PeriodType::Lunch,
					},
					Period {
						friendly_name: "test_period_b".to_string(),
						start: NaiveTime::from_hms_opt(14, 0, 0).unwrap_or_default(),
						start_timestamp: 0,
						end: NaiveTime::from_hms_opt(16, 0, 0).unwrap_or_default(),
						end_timestamp: 0,
						kind: PeriodType::Lunch,
					},
				],
				regex: None,
			},
		);
		result.insert(
			"type_b".to_string(),
			ScheduleType {
				hide: false,
				color: None,
				friendly_name: "Test".to_string(),
				periods: vec![
					Period {
						friendly_name: "test_period_c".to_string(),
						start: NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default(),
						start_timestamp: 0,
						end: NaiveTime::from_hms_opt(13, 0, 0).unwrap_or_default(),
						end_timestamp: 0,
						kind: PeriodType::Lunch,
					},
					Period {
						friendly_name: "test_period_d".to_string(),
						start: NaiveTime::from_hms_opt(15, 0, 0).unwrap_or_default(),
						start_timestamp: 0,
						end: NaiveTime::from_hms_opt(17, 0, 0).unwrap_or_default(),
						end_timestamp: 0,
						kind: PeriodType::Lunch,
					},
				],
				regex: None,
			},
		);
		result.insert(
			"no_school".to_string(),
			ScheduleType {
				hide: false,
				color: None,
				friendly_name: "No School".to_string(),
				periods: vec![],
				regex: Some(Regex::from_str("a").unwrap()),
			},
		);
		result
	};
	schedule.definition.typical_schedule = vec![
		"type_a".to_string(),
		"type_b".to_string(),
		"type_a".to_string(),
		"type_b".to_string(),
		"type_a".to_string(),
		"type_a".to_string(),
		"type_b".to_string(),
	];
	let _ical = IcalEvent::generate(
		&schedule,
		NaiveDate::from_ymd_opt(2020, 1, 1).unwrap_or_default(),
		NaiveDate::from_ymd_opt(2022, 1, 1).unwrap_or_default(),
	);
}
