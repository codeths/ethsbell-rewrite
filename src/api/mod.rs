use rocket::{
	fairing::{Fairing, Info, Kind},
	http::Status,
	response::Responder,
	Response,
};

pub mod v1;

pub struct ApiFairing;

impl Fairing for ApiFairing {
	fn info(&self) -> rocket::fairing::Info {
		Info {
			name: "API methods",
			kind: Kind::Attach,
		}
	}

	fn on_attach(&self, rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
		Ok(rocket.mount("/api/v1", v1::routes()).register(catchers![]))
	}
}

#[derive(thiserror::Error, Debug)]
pub enum OurError {
	#[error("Error trying to interpret date/time string; try YYYY-MM-DD or HH:MM:SS")]
	BadString(#[from] chrono::ParseError),
}

impl<'r> Responder<'r> for OurError {
	fn respond_to(self, request: &rocket::Request) -> rocket::response::Result<'r> {
		Response::build_from(self.to_string().respond_to(request).unwrap())
			.status(match self {
				OurError::BadString(_) => Status::BadRequest,
			})
			.ok()
	}
}
