/// Type aliases for API V1
pub mod v1 {
	use crate::schedule::Period;
	/// This is the type of the data returned by /api/v1/today/now/near.
	pub type NearbyPeriods = (Option<Period>, Vec<Period>, Option<Period>);
}

/// Type aliases for API V2
pub mod v2 {
	use crate::schedule::Period;
	use serde::Serialize;
	use schemars::JsonSchema;

	/// This is the type of the data returned by /api/v2/today/now/near.
	/// The periods returned by `previous` and `future` will have either the
	/// same end time (for `previous`) or the same start time (for `future`)
	/// if there are multiple periods.
	#[derive(Serialize, JsonSchema)]
	pub struct NearbyPeriods {
		/// The `Period`(s) that occurred before this one
		pub previous: Vec<Period>,
		/// The `Period`(s) that are going on right now
		pub current: Vec<Period>,
		/// The `Period`(s) that will occur after this one
		pub future: Vec<Period>,
	}
}
