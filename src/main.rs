#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

//! ETHSBell is a web-based bell schedule tracker.
pub use locks::SpecLock;
use schedule::{get_schedule_from_config, Schedule};

mod aliases;
#[cfg(feature = "ws")]
mod api;
mod frontend;
mod ical;
mod impls;
mod locks;
mod login;
pub mod rocket_builder;
mod schedule;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate schemars;
#[allow(dead_code)]
fn main() {
	// Build the schedule definition and do our first update.
	let schedule = {
		let schedule_def = get_schedule_from_config();

		// Build the runtime schedule struct and run the first update.
		Schedule::from(schedule_def)
	};
	rocket::tokio::runtime::Runtime::new()
		.expect("Failed to open Tokio runtime")
		.block_on(rocket_builder::rocket(schedule).launch())
		.expect("Rocket failed");
}
