use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct SpecLock(pub Option<DateTime<Local>>);
