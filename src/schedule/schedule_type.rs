use chrono::Local;
use chrono::NaiveTime;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::Period;
use super::PeriodType;

/// A type of schedule that can occur.
#[cfg_attr(feature = "ws", derive(JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduleType {
	/// A human-friendly name for the schedule, like "No School" or "E-Learning: Blue Day"
	pub friendly_name: String,
	/// A list of the periods in the day.
	pub periods: Vec<Period>,
	/// The regular expression which calendar entries for this schedule match
	#[cfg_attr(feature = "ws", schemars(skip))]
	#[serde(with = "serde_regex")]
	pub regex: Option<Regex>,
	/// The color of the schedule as RGB, for use in frontends.
	pub color: Option<[u8; 3]>,
	/// Whether the schedule type should be hidden by frontends
	#[serde(default)]
	pub hide: bool,
}
impl ScheduleType {
	/// Returns a tuple of the previous Period, a Vec<Period> of the current periods, and the next Period.
	pub fn at_time(&self, time: NaiveTime) -> (Option<Period>, Vec<Period>, Option<Period>) {
		if self.periods.is_empty() {
			(None, vec![], None)
		} else {
			let mut before: Option<Period> = None;
			let mut current: Vec<Period> = vec![];
			let mut next: Option<Period> = None;
			self.periods.iter().for_each(|period| {
				if period.end <= time {
					if period.kind != PeriodType::Passing {
						match before.clone() {
							Some(before_) if before_.end < period.end => {
								before = Some(period.clone())
							}
							None => before = Some(period.clone()),
							_ => {}
						}
					}
				} else if period.start > time {
					if period.kind != PeriodType::Passing {
						match next.clone() {
							Some(next_) if next_.start > period.start => {
								next = Some(period.clone())
							}
							None => next = Some(period.clone()),
							_ => {}
						}
					}
				} else {
					current.push(period.clone());
				}
			});
			match (&before, &current, &next) {
				(Some(before), v, Some(next)) if v.is_empty() => {
					current = vec![Period {
						friendly_name: "Passing Period".to_string(),
						start: before.end,
						end: next.start,
						start_timestamp: 0,
						end_timestamp: 0,
						kind: PeriodType::Passing,
					}]
				}
				(None, v, Some(next)) if v.is_empty() => {
					current = vec![Period {
						friendly_name: "Before School".to_string(),
						start: NaiveTime::from_hms(0, 0, 0),
						end: next.start,
						start_timestamp: 0,
						end_timestamp: 0,
						kind: PeriodType::BeforeSchool,
					}]
				}
				(Some(before), v, None) if v.is_empty() => {
					current = vec![Period {
						friendly_name: "After School".to_string(),
						start: before.end,
						end: NaiveTime::from_hms(23, 59, 59),
						start_timestamp: 0,
						end_timestamp: 0,
						kind: PeriodType::AfterSchool,
					}]
				}
				_ => {}
			}
			let now = Local::now();
			(
				before.map(|v| v.populate(now)),
				current
					.iter()
					.map(|v| v.clone().populate(now))
					.collect::<Vec<Period>>(),
				next.map(|v| v.populate(now)),
			)
		}
	}
	/// Returns the first period of the schedule with the kind Class(_).
	pub fn first_class(&self) -> Option<Period> {
		self.periods
			.iter().find(|v| matches!(v.kind, PeriodType::Class(_)))
			.cloned()
	}
}

impl PartialEq for ScheduleType {
	fn eq(&self, other: &Self) -> bool {
		self.friendly_name == other.friendly_name
			&& self.periods == other.periods
			&& self.color == other.color
			&& self.regex.clone().map(|v| v.to_string())
				== other.regex.clone().map(|v| v.to_string())
	}
}
