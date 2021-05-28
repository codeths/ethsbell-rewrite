use rocket::{
	fairing::{Fairing, Info, Kind},
	http::Status,
	response::{status::BadRequest, Responder},
	Response,
};
use rocket_okapi::{
	response::OpenApiResponder,
	swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

use crate::login::WantsBasicAuth;
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
		Ok(rocket
			.mount("/api/v1", v1::routes())
			.mount(
				"/docs/v1",
				make_swagger_ui(&SwaggerUIConfig {
					url: "../../api/v1/openapi.json".to_owned(),
					..Default::default()
				}),
			)
			.register(catchers![wants_auth]))
	}
}

#[catch(401)]
fn wants_auth() -> WantsBasicAuth {
	WantsBasicAuth
}

// This shows an error in rust-analyzer but it doesn't actually fail to compile?
#[derive(thiserror::Error, Debug, JsonSchema)]
pub enum OurError {
	#[error("Error trying to interpret date/time string; try YYYY-MM-DD or HH:MM:SS")]
	#[schemars(skip)]
	BadString(#[from] chrono::ParseError),
	#[error("Error trying to access a file")]
	#[schemars(skip)]
	IOError(#[from] std::io::Error),
	#[error("Error trying to transform some data")]
	#[schemars(skip)]
	SerdeError(#[from] serde_json::Error),
}

impl<'r> Responder<'r> for OurError {
	fn respond_to(self, request: &rocket::Request) -> rocket::response::Result<'r> {
		Response::build_from(self.to_string().respond_to(request).unwrap())
			.status(match self {
				OurError::BadString(_) => Status::BadRequest,
				OurError::IOError(_) => Status::InternalServerError,
				OurError::SerdeError(_) => Status::BadRequest,
			})
			.ok()
	}
}

impl<'r> OpenApiResponder<'r> for OurError {
	fn responses(
		gen: &mut rocket_okapi::gen::OpenApiGenerator,
	) -> rocket_okapi::Result<okapi::openapi3::Responses> {
		BadRequest::<()>::responses(gen)
	}
}
