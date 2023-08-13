//! This file defines our authentication behavior for the admin editor.

#[cfg(feature = "ws")]
use rocket::request::Outcome;
#[cfg(feature = "ws")]
use rocket::response::Response;
#[cfg(feature = "ws")]
use rocket::{http::Status, request::FromRequest, response::Responder};
#[cfg(feature = "ws")]
use rocket_okapi::request::OpenApiFromRequest;
#[cfg(feature = "ws")]
use std::env;
#[cfg(feature = "ws")]
use std::io::Cursor;
/// This struct is used as a request guard to require authentication.
#[cfg_attr(feature = "ws", derive(OpenApiFromRequest))]
#[derive(Clone, Copy)]
pub struct Authenticated;

#[cfg(feature = "ws")]
#[async_trait]
impl<'r> FromRequest<'r> for Authenticated {
	type Error = WantsBasicAuth;

	async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
		let auth = request.headers().get_one("Authorization");
		match auth {
			Some(auth) if auth.starts_with("Basic") => {
				let auth = match base64::decode(auth.chars().skip(6).collect::<String>()) {
					Ok(bytes) => String::from_utf8(bytes).unwrap_or_else(|_| "ERROR".to_string()),
					Err(_) => "ERROR".to_string(),
				};
				let (username, password) = {
					let mut split = auth.split(':');
					(split.next().unwrap(), split.next().unwrap())
				};
				if username == env::var("BELL_USERNAME").expect("Set BELL_USERNAME please")
					&& password == env::var("BELL_PASSWORD").expect("Set BELL_PASSWORD please")
				{
					Outcome::Success(Authenticated)
				} else {
					Outcome::Failure((Status::Unauthorized, WantsBasicAuth))
				}
			}
			_ => Outcome::Failure((Status::Unauthorized, WantsBasicAuth)),
		}
	}
}

/// This struct defines an error type which returns the corresponding HTTP error code.
#[derive(Debug)]
pub struct WantsBasicAuth;

#[cfg(feature = "ws")]
impl<'o, 'r> Responder<'o, 'r> for WantsBasicAuth
where
	'r: 'o,
{
	fn respond_to(self, _request: &rocket::Request) -> rocket::response::Result<'r> {
		Response::build()
			.streamed_body(Cursor::new("Needs authorization"))
			.ok()
	}
}
