#![feature(decl_macro)]

use std::collections::HashMap;
#[cfg(feature = "ws")]
use std::{env, fs};

use schedule::{Schedule, ScheduleDefinition, ScheduleType};

mod api;
mod frontend;
mod ical;
mod impls;
mod locks;
mod login;
pub mod rocket_builder;
mod schedule;
pub use locks::SpecLock;
use serde_json::Value;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate schemars;
#[allow(dead_code)]
fn main() {
	// Build the schedule definition and do our first update.
	let schedule = {
		// Load the definition.
		let string = if cfg!(target_arch = "wasm32") {
			include_str!("../def.json").to_string()
		} else {
			fs::read_to_string(
				env::var("SCHEDULE_DEF").unwrap_or_else(|_| "./def.json".to_string()),
			)
			.expect("Opened schedule definition")
		};

		// Deserialize the definition.
		let mut schedule_def: ScheduleDefinition =
			serde_json::from_str(&string).expect("Deserialized schedule definition");

		// Load maps from def.d
		let extra_maps: Vec<HashMap<String, ScheduleType>> = match fs::read_dir("./def.d") {
			Ok(listing) => {
				let mut out = vec![];
				for file in listing
					.map(|v| v.expect("Couldn't list def.d").path())
					.filter(|v| v.to_str().unwrap().ends_with(".json"))
				{
					println!("{}", file.to_str().unwrap());
					let string = fs::read_to_string(file).expect("Couldn't read a schedule file");
					let mut map: HashMap<String, Value> =
						serde_json::from_str(&string).expect("Couldn't interpret a schedule file");
					map.remove(&"$schema".to_string());
					let map = map
						.iter()
						.map(|(k, v)| {
							(
								k.clone(),
								serde_json::from_value::<ScheduleType>(v.clone()).unwrap(),
							)
						})
						.collect::<HashMap<String, ScheduleType>>();
					out.push(map)
				}
				out
			}
			Err(_) => vec![],
		};
		// Apply maps
		for map in extra_maps {
			for (k, v) in map {
				schedule_def.schedule_types.insert(k, v);
			}
		}

		// Build the runtime schedule struct and run the first update.
		Schedule::from(schedule_def)
	};
	rocket_builder::rocket(schedule).launch();
}
