#![feature(decl_macro)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

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
	rocket_builder::rocket(schedule).launch();
}
