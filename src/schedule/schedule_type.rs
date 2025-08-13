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
	#[must_use]
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
								before = Some(period.clone());
							}
							None => before = Some(period.clone()),
							_ => {}
						}
					}
				} else if period.start > time {
					if period.kind != PeriodType::Passing {
						match next.clone() {
							Some(next_) if next_.start > period.start => {
								next = Some(period.clone());
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
					}];
				}
				(None, v, Some(next)) if v.is_empty() => {
					current = vec![Period {
						friendly_name: "Before School".to_string(),
						start: NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_default(),
						end: next.start,
						start_timestamp: 0,
						end_timestamp: 0,
						kind: PeriodType::BeforeSchool,
					}];
				}
				(Some(before), v, None) if v.is_empty() => {
					current = vec![Period {
						friendly_name: "After School".to_string(),
						start: before.end,
						end: NaiveTime::from_hms_opt(23, 59, 59).unwrap_or_default(),
						start_timestamp: 0,
						end_timestamp: 0,
						kind: PeriodType::AfterSchool,
					}];
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

	/// Returns a tuple of a Vec<Period> of the previous Periods, a Vec<Period> of the current periods,
	/// and a Vec<Period> of the next periods.
	#[must_use]
	pub fn at_time_v2(&self, time: NaiveTime) -> (Vec<Period>, Vec<Period>, Vec<Period>) {
		if self.periods.is_empty() {
			(vec![], vec![], vec![])
		} else {
			let mut previous: Vec<Period> = vec![];
			let mut current: Vec<Period> = vec![];
			let mut future: Vec<Period> = vec![];

			// sort periods into previous, current, and future
			self.periods.iter().for_each(|period| {
				if period.end <= time {
					if period.kind != PeriodType::Passing {
						match previous.get(0).clone() {
							Some(before) => {
								if period.end > before.end {
									// if we find a more recent period, replace everything
									previous = vec![period.clone()];
								} else if period.end == before.end {
									// if we find a matching end time, add it
									previous.push(period.clone());
								}
							}
							None => previous.push(period.clone()),
						}
					}
				} else if period.start > time {
					if period.kind != PeriodType::Passing {
						match future.get(0).clone() {
							Some(after) => {
								if period.start < after.start {
									future = vec![period.clone()];
								} else if period.start == after.start {
									future.push(period.clone());
								}
							}
							None => future.push(period.clone()),
						}
					}
				} else {
					current.push(period.clone());
				}
			});

			// add implicit periods
			if current.is_empty() {
				match (previous.is_empty(), future.is_empty()) {
					(false, false) => {
						current = vec![Period {
							friendly_name: "Passing Period".to_string(),
							start: previous[0].end,
							end: future[0].start,
							start_timestamp: 0,
							end_timestamp: 0,
							kind: PeriodType::Passing,
						}];
					}
					(true, false) => {
						current = vec![Period {
							friendly_name: "Before School".to_string(),
							start: NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_default(),
							end: future[0].start,
							start_timestamp: 0,
							end_timestamp: 0,
							kind: PeriodType::BeforeSchool,
						}];
					}
					(false, true) => {
						current = vec![Period {
							friendly_name: "After School".to_string(),
							start: previous[0].end,
							end: NaiveTime::from_hms_opt(23, 59, 59).unwrap_or_default(),
							start_timestamp: 0,
							end_timestamp: 0,
							kind: PeriodType::AfterSchool,
						}];
					}
					_ => {}
				}
			}

			// populate timestamps
			let now = Local::now();
			(
				previous
					.iter()
					.map(|v| v.clone().populate(now))
					.collect::<Vec<Period>>(),
				current
					.iter()
					.map(|v| v.clone().populate(now))
					.collect::<Vec<Period>>(),
				future
					.iter()
					.map(|v| v.clone().populate(now))
					.collect::<Vec<Period>>(),
			)
		}
	}

	/// Returns the first period of the schedule with the kind Class(_).
	#[must_use]
	pub fn first_class(&self) -> Option<Period> {
		self.periods
			.iter()
			.find(|v| matches!(v.kind, PeriodType::Class(_)))
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
