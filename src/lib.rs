#[cfg(feature = "ws")]
#[macro_use]
extern crate rocket_okapi;

pub mod ical;
pub mod schedule;
pub mod impls;

/// Re-export to allow for typed deserialization in js
pub use serde_json::from_str;
