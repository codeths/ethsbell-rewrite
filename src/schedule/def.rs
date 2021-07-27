use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::ScheduleType;

/// The definition of the schedule.
#[cfg_attr(feature = "ws", derive(JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduleDefinition {
	/// The URL of the ical calendar we fetch the schedule's data from, in ascending order of importance
	pub calendar_urls: Vec<String>,
	/// All of the types of schedule there are.
	pub schedule_types: HashMap<String, ScheduleType>,
	/// The typical schedule.
	/// Should have seven elements, starting on Sunday and ending on Saturday.
	pub typical_schedule: Vec<String>,
}
