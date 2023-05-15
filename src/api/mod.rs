//! Defines backend behavior.

#[cfg(feature = "ws")]
use rocket::{
	fairing::{Fairing, Info, Kind},
	http::Status,
	response::{status::BadRequest, Responder},
	Response,
};
#[cfg(feature = "ws")]
use rocket_okapi::{
	response::OpenApiResponder,
	swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

#[cfg(feature = "ws")]
use crate::login::WantsBasicAuth;
#[cfg(feature = "ws")]
pub mod legacy;
pub mod v1;

/// This struct is used as a Rocket Fairing and adds our API endpoints.
#[cfg(feature = "ws")]
pub struct ApiFairing;

#[cfg(feature = "ws")]
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
			.mount("/api/legacy", legacy::routes())
			.mount(
				"/docs/v1",
				make_swagger_ui(&SwaggerUIConfig {
					url: "../../api/v1/openapi.json".to_owned(),
					..Default::default()
				}),
			)
			.mount(
				"/docs/legacy",
				make_swagger_ui(&SwaggerUIConfig {
					url: "../../api/legacy/openapi.json".to_owned(),
					..Default::default()
				}),
			)
			.register(catchers![wants_auth]))
	}
}

#[cfg(feature = "ws")]
#[catch(401)]
fn wants_auth() -> WantsBasicAuth {
	WantsBasicAuth
}

/// This defines how we convert Errors into Responses
#[cfg_attr(feature = "ws", derive(JsonSchema))]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum OurError {
	#[error("Error trying to interpret date/time string; try YYYY-MM-DD or HH:MM:SS")]
	#[cfg_attr(feature = "ws", schemars(skip))]
	BadString(#[from] chrono::ParseError),
	#[error("Error trying to access a file")]
	#[cfg_attr(feature = "ws", schemars(skip))]
	IOError(#[from] std::io::Error),
	#[error("Error trying to transform some data")]
	#[cfg_attr(feature = "ws", schemars(skip))]
	SerdeError(#[from] serde_json::Error),
}

#[cfg(feature = "ws")]
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

#[cfg(feature = "ws")]
impl<'r> OpenApiResponder<'r> for OurError {
	fn responses(
		gen: &mut rocket_okapi::gen::OpenApiGenerator,
	) -> rocket_okapi::Result<okapi::openapi3::Responses> {
		BadRequest::<()>::responses(gen)
	}
}

/// Dummy state struct for library use
#[cfg(not(feature = "ws"))]
pub struct State<T>(T);

#[cfg(not(feature = "ws"))]
impl<T> std::ops::Deref for State<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

/// Dummy json struct for library use
#[cfg(not(feature = "ws"))]
pub struct Json<T>(T);

#[cfg(not(feature = "ws"))]
impl<T> std::ops::Deref for Json<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}
