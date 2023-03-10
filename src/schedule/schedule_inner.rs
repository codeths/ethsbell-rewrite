use chrono::Datelike;
use chrono::{Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::{env, fs};

use std::path::Path;
use std::sync::{Arc, RwLock};
#[cfg(feature = "ws")]
use std::thread;

#[cfg(feature = "pull")]
use super::{ical_to_ours, IcalEvent};
use super::{Event, ScheduleDefinition, ScheduleType};

/// A structure containing all of the information we need to operate.
#[cfg_attr(feature = "ws", derive(JsonSchema))]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Schedule {
	/// When this schedule's calendar data was last updated.
	pub last_updated: NaiveDateTime,
	/// A map from every date to the list of events occurring on that date.
	pub calendar: HashMap<NaiveDate, Vec<Event>>,
	/// The definition from which this schedule was built.
	pub definition: ScheduleDefinition,
}
#[cfg(feature = "pull")]
impl From<ScheduleDefinition> for Schedule {
	/// Creates a new Schedule and performs the first update.
	fn from(def: ScheduleDefinition) -> Self {
		let mut new = Schedule {
			last_updated: NaiveDateTime::from_timestamp(0, 0),
			calendar: HashMap::new(),
			definition: def,
		};
		new.update();
		new
	}
}
impl Default for Schedule {
	/// Returns an empty schedule with all default properties.
	fn default() -> Self {
		Schedule {
			last_updated: NaiveDateTime::from_timestamp(0, 0),
			calendar: HashMap::new(),
			definition: ScheduleDefinition {
				calendar_urls: vec![],
				schedule_types: HashMap::new(),
				typical_schedule: vec![],
			},
		}
	}
}
impl Schedule {
	/// Updates the schedule only if needed, locking minimally.
	#[cfg(feature = "pull")]
	pub fn update_if_needed_async(schedule: Arc<RwLock<Schedule>>) {
		if schedule.read().unwrap().is_update_needed() {
			schedule.write().unwrap().last_updated = Local::now().naive_local();
			thread::spawn(|| Schedule::update_async(schedule));
		}
	}
	/// Dummy function to allow compiling without pull.
	#[cfg(not(feature = "pull"))]
	pub fn update_if_needed_async(_schedule: Arc<RwLock<Schedule>>) {}
	/// Updates a schedule, locking minimally.
	#[cfg(feature = "pull")]
	pub fn update_async(schedule: Arc<RwLock<Schedule>>) {
		println!("Refreshing...");
		// Fetch the calendars
		let calendars = schedule
			.read()
			.unwrap()
			.definition
			.calendar_urls
			.iter()
			.map(|v| IcalEvent::get(v))
			.collect::<Vec<Vec<IcalEvent>>>();
		for cal in calendars {
			ical_to_ours(&mut schedule.write().unwrap(), &cal)
		}
		// Update the last-updated value
		schedule.write().unwrap().last_updated = Local::now().naive_local();
		println!("Done.");
	}
	/// Updates the schedule, requiring a lock for the whole process.
	#[cfg(feature = "pull")]
	pub fn update(&mut self) {
		println!("Refreshing...");
		// Fetch the calendars
		for cal in self.definition.calendar_urls.clone() {
			ical_to_ours(self, &IcalEvent::get(&cal))
		}
		// Update the last-updated value
		self.last_updated = Local::now().naive_local();
		println!("Done.");
	}
	/// Returns whether the schedule's calendar data is out of date.
	pub fn is_update_needed(&self) -> bool {
		match env::var("UPDATE_INTERVAL") {
			Ok(v) => {
				let seconds: u64 = v.parse().unwrap();
				let latest_needed = Local::now().naive_local().timestamp() as u64 - seconds;
				let last_updated = self.last_updated.timestamp() as u64;
				latest_needed > last_updated
			}
			Err(_) => self.last_updated.date() != Local::now().date().naive_local(),
		}
	}
	/// Returns a tuple of the schedule occurring on a target date and its key in the schedule table.
	pub fn on_date(&self, date: NaiveDate) -> (ScheduleType, Option<String>) {
		let mut literal: Option<ScheduleType> = None;
		let special: Option<String> = self
			.calendar
			.iter()
			.filter(|v| v.0 == &date)
			.map(|v| {
				for i in v.1 {
					match i {
						Event::ScheduleOverride(s) => {
							return Some(s);
						}
						Event::ScheduleLiteral(s) => {
							literal = Some(serde_json::from_str(s).unwrap());
							return None;
						}
						_ => {}
					}
				}
				None
			})
			.filter(|v| v.is_some())
			.map(|v| v.unwrap().clone())
			.next();
		match special {
			Some(name) => (
				self.definition.schedule_types.get(&name).unwrap().clone(),
				Some(name),
			),
			None => match literal {
				Some(schedule) => (schedule, None),
				None => {
					let weekday: usize = date.weekday().num_days_from_sunday().try_into().unwrap();
					let name = self.definition.typical_schedule[weekday].clone();
					(
						self.definition.schedule_types.get(&name).unwrap().clone(),
						Some(name),
					)
				}
			},
		}
	}
}

/// Get schedule JSON from definition file
pub fn get_schedule_from_config() -> ScheduleDefinition {
	if !Path::new("./def.json").exists() {
		fs::copy("./def.example.json", "./def.json").expect("Could not copy def");
	}

	let string =
		fs::read_to_string(env::var("SCHEDULE_DEF").unwrap_or_else(|_| "./def.json".to_string()))
			.expect("Opened schedule definition");

	// Deserialize the definition.
	let schedule_def: ScheduleDefinition =
		serde_json::from_str(&string).expect("Deserialized schedule definition");
	schedule_def
}
