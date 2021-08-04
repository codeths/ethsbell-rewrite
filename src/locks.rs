//! This file contains structs used to persist lock states between requests.
use chrono::{DateTime, Local};
/// This is a struct used to persist the admin editor's lock state between requests.
#[derive(Clone)]
pub struct SpecLock(pub Option<DateTime<Local>>);
