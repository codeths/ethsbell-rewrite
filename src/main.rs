#![feature(decl_macro)]

use std::collections::HashMap;
#[cfg(feature = "ws")]
use std::{
	env, fs,
	sync::{Arc, Mutex, RwLock},
};

use chrono::{DateTime, Local};
use rocket_contrib::templates::Template;
use schedule::{Schedule, ScheduleDefinition, ScheduleType};

mod api;
mod frontend;
mod ical;
mod impls;
mod locks;
mod login;
mod schedule;
pub use locks::SpecLock;

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
			fs::read_to_string(env::var("SCHEDULE_DEF").unwrap_or("./def.json".to_string()))
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
					let map =
						serde_json::from_str(&string).expect("Couldn't interpret a schedule file");
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
		let schedule = Schedule::from(schedule_def);
		// Wrap the runtime schedule struct in a thread-safe container.
		Arc::new(RwLock::new(schedule))
	};
	let spec_lock = Arc::new(Mutex::new(SpecLock(None)));
	rocket::ignite()
		.attach(api::ApiFairing)
		.attach(frontend::FrontendFairing)
		.attach(Template::fairing())
		.manage(schedule.clone())
		.manage(spec_lock)
		.launch();
}
