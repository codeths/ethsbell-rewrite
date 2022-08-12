use chrono::{Datelike, Duration, NaiveDate, Timelike};
use rand::Rng;
#[cfg(feature = "pull")]
use reqwest::blocking::get;
use serde::Deserialize;

use crate::schedule::Schedule;

/// An event in iCal
#[derive(Deserialize)]
pub struct IcalEvent {
	/// The event's summary.
	pub summary: Option<String>,
	/// The event's description.
	pub description: Option<String>,
	/// The start of the event.
	pub start: Option<NaiveDate>,
	/// The end of the event.
	pub end: Option<NaiveDate>,
}
impl IcalEvent {
	/// Download a Vec<IcalEvent> from the provided URL.
	#[cfg(feature = "pull")]
	pub fn get(url: &str) -> Vec<IcalEvent> {
		let data = get(url).unwrap().text().unwrap();
		IcalEvent::from_string(&data)
	}
	/// Parse a Vec<IcalEvent> from the provided string.
	pub fn from_string(data: &str) -> Vec<IcalEvent> {
		data.split("BEGIN:VEVENT")
			.map(|v| v.trim())
			.map(|vevent| {
				let mut result = IcalEvent {
					summary: None,
					description: None,
					start: None,
					end: None,
				};
				for (number, line) in vevent.lines().enumerate() {
					let mut split = line.split(':');
					let kind = split.next();
					match kind {
						Some(kind) if kind.starts_with("DTSTART") => {
							let string = split.next().unwrap().chars();
							let year: i32 =
								string.clone().take(4).collect::<String>().parse().unwrap();
							let month: u32 = string
								.clone()
								.skip(4)
								.take(2)
								.collect::<String>()
								.parse()
								.unwrap();
							let day: u32 = string
								.clone()
								.skip(6)
								.take(2)
								.collect::<String>()
								.parse()
								.unwrap();
							result.start = Some(NaiveDate::from_ymd(year, month, day));
						}
						Some(kind) if kind.starts_with("DTEND") => {
							let string = split.next().unwrap().chars();
							let year: i32 =
								string.clone().take(4).collect::<String>().parse().unwrap();
							let month: u32 = string
								.clone()
								.skip(4)
								.take(2)
								.collect::<String>()
								.parse()
								.unwrap();
							let day: u32 = string
								.clone()
								.skip(6)
								.take(2)
								.collect::<String>()
								.parse()
								.unwrap();
							result.end = Some(NaiveDate::from_ymd(year, month, day));
						}
						Some("DURATION") => {
							let days = split
								.next()
								.unwrap()
								.chars()
								.filter(|v| v.is_digit(10))
								.collect::<String>()
								.parse()
								.unwrap();
							result.end = Some(result.start.unwrap() + Duration::days(days));
						}
						Some("SUMMARY") => result.summary = Some(split.next().unwrap().to_string()),
						Some("DESCRIPTION") => {
							let other_lines = vevent
								.lines()
								.skip(number + 1)
								.take_while(|v| v.starts_with('\t') | v.starts_with(' '))
								.map(|v| v.trim_start())
								.collect::<String>();
							let text = (split.collect::<Vec<&str>>().join(":") + &other_lines)
								.to_string()
								.replace("\\,", ",");
							result.description = Some(text)
						}
						Some(_) => {}
						None => {}
					}
				}
				result
			})
			.filter(|v| (v.description != None || v.summary != None) && v.start != None)
			.collect()
	}
	/// Generate a semi-valid ICal file from a Schedule for the given date range.
	pub fn generate(schedule: &Schedule, start: NaiveDate, end: NaiveDate) -> String {
		let mut rng = rand::thread_rng();
		let mut result = String::new();
		result += "BEGIN:VCALENDAR
VERSION:2.0
PRODID:ETHSBell Rewrite
";
		let mut exception_days: Vec<NaiveDate> = vec![];
		// Populate special events
		for (date, events) in &schedule.calendar {
			if date < &start || date > &end {
				continue;
			}
			// Populate the day's schedule
			let is_special = {
				let mut output = true;
				for event in events {
					match event {
						crate::schedule::Event::ScheduleOverride(_) => output = false,
						crate::schedule::Event::ScheduleLiteral(_) => output = false,
						crate::schedule::Event::SpecialEvent(_) => {}
					}
				}
				output
			};
			if is_special {
				continue;
			}
			exception_days.push(*date);
			let day = schedule.on_date(*date);
			for period in day.0.periods {
				result += &format!(
					"BEGIN:VEVENT
UID:{uid}
SUMMARY:{summary}
DTSTAMP:{dtstamp}
DTSTART:{dtstart}
DTEND:{dtend}
END:VEVENT
",
					uid = rng.gen::<usize>(),
					summary = period.friendly_name,
					dtstamp = format_args!(
						"{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}",
						date.year(),
						date.month(),
						date.day(),
						period.start.hour(),
						period.start.minute(),
						period.start.second()
					),
					dtstart = format_args!(
						"{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}",
						date.year(),
						date.month(),
						date.day(),
						period.start.hour(),
						period.start.minute(),
						period.start.second()
					),
					dtend = format_args!(
						"{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}",
						date.year(),
						date.month(),
						date.day(),
						period.end.hour(),
						period.end.minute(),
						period.end.second()
					),
				);
			}
		}
		// Populate the typical schedules
		for (index, id) in schedule.definition.typical_schedule.iter().enumerate() {
			let schedule_type = schedule
				.definition
				.schedule_types
				.get(id)
				.expect("Invalid typical schedule type");
			for period in &schedule_type.periods {
				result += &format!(
					"BEGIN:VEVENT
UID:{uid}
DTSTAMP:{dtstamp}
DTSTART:{dtstart}
DTEND:{dtend}
SUMMARY:{summary}
RRULE:FREQ=WEEKLY;BYDAY={day}
EXDATE:{exdate}
END:VEVENT
",
					day = ["SU", "MO", "TU", "WE", "TH", "FR", "SA"][index],
					uid = rng.gen::<usize>(),
					summary = period.friendly_name,
					dtstart = format_args!(
						"{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}",
						start.year(),
						start.month(),
						start.day(),
						period.start.hour(),
						period.start.minute(),
						period.start.second()
					),
					dtstamp = format_args!(
						"{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}",
						start.year(),
						start.month(),
						start.day(),
						period.start.hour(),
						period.start.minute(),
						period.start.second()
					),
					dtend = format_args!(
						"{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}",
						start.year(),
						start.month(),
						start.day(),
						period.end.hour(),
						period.end.minute(),
						period.end.second()
					),
					exdate = exception_days
						.iter()
						.map(|v| format!("{:0>4}{:0>2}{:0>2}T000000", v.year(), v.month(), v.day()))
						.collect::<Vec<String>>()
						.join(",")
				);
			}
		}
		result += "END:VCALENDAR\n";
		result
	}
}
