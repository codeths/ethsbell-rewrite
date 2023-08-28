//! Defines frontend behavior.

use rocket::{
	fairing::{Fairing, Info, Kind},
	fs::FileServer,
	Build,
};

/// A struct that is used as a Rocket Fairing to load the frontend from disk.
pub struct FrontendFairing;

#[async_trait]
impl Fairing for FrontendFairing {
	fn info(&self) -> rocket::fairing::Info {
		Info {
			name: "Frontend",
			kind: Kind::Ignite,
		}
	}

	async fn on_ignite(
		&self,
		rocket: rocket::Rocket<Build>,
	) -> Result<rocket::Rocket<Build>, rocket::Rocket<Build>> {
		Ok(rocket.mount("/", FileServer::from("./frontend-dist")))
	}
}
