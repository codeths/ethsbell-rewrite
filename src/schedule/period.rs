use chrono::{DateTime, Local, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

/// The definition for a period.
#[cfg_attr(feature = "ws", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Period {
	/// A human-friendly name for this period, like "First Period".
	pub friendly_name: String,
	/// The start of this period.
	pub start: NaiveTime,
	/// The start of this period, formatted as the Unix epoch.
	/// This field is zero in most responses.
	#[serde(skip_deserializing)]
	pub start_timestamp: u64,
	/// The end of this period.
	pub end: NaiveTime,
	/// The end of this period, formatted as the Unix epoch.
	/// This field is zero in most responses.
	#[serde(skip_deserializing)]
	pub end_timestamp: u64,
	/// The type of this period.
	pub kind: PeriodType,
}
impl Period {
	/// Populate a period's timestamp fields, which are normally 0.
	pub fn populate(mut self, date: DateTime<Local>) -> Self {
		let start_date = date.clone();
		let end_date = date.clone();
		self.start_timestamp = start_date
			.with_hour(self.start.hour())
			.unwrap()
			.with_minute(self.start.minute())
			.unwrap()
			.with_second(self.start.second())
			.unwrap()
			.timestamp() as u64;
		self.end_timestamp = end_date
			.with_hour(self.end.hour())
			.unwrap()
			.with_minute(self.end.minute())
			.unwrap()
			.with_second(self.end.second())
			.unwrap()
			.timestamp() as u64;
		self
	}
}

/// The types a period can be.
#[cfg_attr(feature = "ws", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum PeriodType {
	/// This period has a class in it, and it is this index in a student's schedule.
	Class(String),
	/// This period is either a lunch or a class, depending on the student's schedule.
	ClassOrLunch(String),
	/// This period is always lunch.
	Lunch,
	/// This period is always a break.
	Break,
	/// This period is AM support.
	AMSupport,
	/// This period is always a passing period.
	/// Declaring this is optional; frontend applications should automatically report a passing period when between two existing periods.
	Passing,
	/// This period is before school.
	BeforeSchool,
	/// This period is after school.
	AfterSchool,
	/// This period contains announcements.
	Announcements,
}
