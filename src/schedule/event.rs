use chrono::Duration;
use serde::Serialize;

use super::{IcalEvent, Schedule, ScheduleType};

/// Types of calendar events.
#[allow(clippy::enum_variant_names)]
#[cfg_attr(feature = "ws", derive(JsonSchema))]
#[derive(Serialize, Clone, PartialEq, Debug)]
pub enum Event {
	/// This variant causes an override of the current schedule to the schedule named in the variant.
	ScheduleOverride(String),
	/// This variant causes the schedule contained within to be used
	ScheduleLiteral(String),
	/// This variant causes a special event message to be included in the API response.
	SpecialEvent(String),
}

/// Write a Vec<IcalEvent> to our runtime schedule struct.
pub fn ical_to_ours(schedule: &mut Schedule, data: &[IcalEvent]) {
	// For every ical event...
	data.iter().for_each(|event| {
		let start = event.start.unwrap();
		// The end is either 1 day after the start or equal to the defined end.
		let mut end = event.end.unwrap_or(start + Duration::days(1));
		// If the defined end is on the same day, we'll pretend it's the next day.
		if end == start {
			end += Duration::days(1)
		}
		// Start on the starting date, of course...
		let mut day = start;
		while day < end {
			// Get the calendar's response for the day, whether or not it exists.
			let date = schedule.calendar.get(&day);
			// Create it if it doesn't exist.
			match &date {
				Some(_) => {}
				None => {
					schedule.calendar.insert(day, vec![]);
				}
			}
			// Unwrap the calendar's entry, now that we know it exists.
			let date = schedule.calendar.get_mut(&day).unwrap();
			// Check if the summary is a literal schedule
			let literal_header = "LITERAL SCHEDULE ";
			if event
				.description
				.as_ref()
				.unwrap_or(&"".to_string())
				.starts_with(literal_header)
			{
				let json = event
					.description
					.as_ref()
					.unwrap()
					.to_string()
					.replace("\n ", "")
					.chars()
					.skip(literal_header.len())
					.collect::<String>();
				let result = serde_json::from_str::<ScheduleType>(&json);
				if result.is_ok() {
					date.push(Event::ScheduleLiteral(json));
					return;
				} else {
					println!("Error parsing schedule literal: {:?}", result.unwrap_err())
				}
			}
			// Check against every schedule
			let mut is_schedule_event = false;
			for i in &schedule.definition.schedule_types {
				// If this schedule's regex matches...
				if i.1.regex.is_some()
					&& i.1
						.regex
						.as_ref()
						.unwrap()
						.is_match(event.summary.as_ref().unwrap())
				{
					let mut found = false;
					// Check to see if a special schedule already exists for today...
					for o in date.iter_mut() {
						if let Event::ScheduleOverride(schedule) = o {
							*schedule = i.0.clone();
							found = true;
							is_schedule_event = true
						}
					}
					if !found {
						// Otherwise, create a new event entry.

						// Check if there is a partial literal schedule to apply
						let literal_header = "PARTIAL LITERAL ";
						if event
							.description
							.as_ref()
							.unwrap_or(&"".to_string())
							.starts_with(literal_header)
						{
							// Get original schedule
							let mut json = serde_json::to_value(
								schedule
									.definition
									.schedule_types
									.get(&i.0.clone())
									.unwrap(),
							)
							.unwrap();

							// Get partial schedule from description
							let result = serde_json::from_str(
								&event
									.description
									.as_ref()
									.unwrap()
									.to_string()
									.replace("\n ", "")
									.chars()
									.skip(literal_header.len())
									.collect::<String>(),
							);
							if let Ok(r) = result {
								// Merge partial schedule into original
								merge(&mut json, &r);

								date.push(Event::ScheduleLiteral(
									serde_json::to_string(&json).unwrap(),
								));
								return;
							} else {
								println!(
									"Error parsing schedule literal: {:?}",
									result.unwrap_err()
								)
							}
						} else {
							date.push(Event::ScheduleOverride(i.0.clone()));
							is_schedule_event = true;
						}
					}
				}
			}
			if !is_schedule_event {
				// If this event didn't match any special schedules, add it as a non-schedule Special Event.
				let new_event = Event::SpecialEvent(event.summary.as_ref().unwrap().clone());
				if !date.contains(&new_event) {
					date.push(new_event)
				}
			}
			// Move to the next day in the event.
			day += Duration::days(1)
		}
	})
}

fn merge(a: &mut serde_json::Value, b: &serde_json::Value) {
	match (a, b) {
		(&mut serde_json::Value::Object(ref mut a), &serde_json::Value::Object(ref b)) => {
			for (k, v) in b {
				merge(a.entry(k.clone()).or_insert(serde_json::Value::Null), v);
			}
		}
		(a, b) => {
			*a = b.clone();
		}
	}
}
