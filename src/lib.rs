#[cfg(feature = "ws")]
#[macro_use]
extern crate rocket_okapi;

pub mod ical;
mod impls;
pub mod schedule;

/// Re-export things from serde.
pub use serde_json::from_str;
