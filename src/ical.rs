//! Functions for parsing iCalendar files.
use chrono::{Duration, NaiveDate};
#[cfg(feature = "pull")]
use reqwest::blocking::get;
use serde::Deserialize;

/// An event in iCal
#[derive(Deserialize)]
pub struct IcalEvent {
	pub summary: Option<String>,
	pub description: Option<String>,
	pub start: Option<NaiveDate>,
	pub end: Option<NaiveDate>,
}
impl IcalEvent {
	#[cfg(feature = "pull")]
	pub fn get(url: &String) -> Vec<IcalEvent> {
		let data = get(url).unwrap().text().unwrap();
		IcalEvent::from_string(&data)
	}
	pub fn from_string(data: &String) -> Vec<IcalEvent> {
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
					let mut split = line.split(":");
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
								.take_while(|v| v.starts_with(" "))
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
}
