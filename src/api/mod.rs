use rocket::fairing::{Fairing, Info, Kind};

pub mod v1;

pub struct ApiFairing {}

impl Fairing for ApiFairing {
	fn info(&self) -> rocket::fairing::Info {
		Info {
			name: "API methods",
			kind: Kind::Attach,
		}
	}

	fn on_attach(&self, rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
		Ok(rocket.mount("/api/v1", v1::routes()))
	}
}
