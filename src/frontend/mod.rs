use rocket::fairing::{Fairing, Info, Kind};
use rocket_contrib::serve::StaticFiles;

pub struct FrontendFairing;

impl Fairing for FrontendFairing {
	fn info(&self) -> rocket::fairing::Info {
		Info {
			name: "Frontend",
			kind: Kind::Attach,
		}
	}

	fn on_attach(&self, rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
		Ok(rocket.mount("/", StaticFiles::from("./frontend-dist")))
	}
}
