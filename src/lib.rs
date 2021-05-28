pub mod ical;
pub mod schedule;

/// Re-export to allow for typed deserialization in js
pub use serde_json::from_str;
