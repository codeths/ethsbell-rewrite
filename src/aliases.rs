/// Type aliases for API V1
pub mod v1 {
	use crate::schedule::Period;
	/// This is the type of the data returned by /api/v1/today/now/near.
	pub type NearbyPeriods = (Option<Period>, Vec<Period>, Option<Period>);
}

/// Type aliases for API V2
pub mod v2 {
	use crate::schedule::Period;
	/// This is the type of the data returned by /api/v2/today/now/near.
	pub type NearbyPeriods = (Vec<Period>, Vec<Period>, Option<Period>);
}
