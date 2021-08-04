//! Functions for parsing iCalendar files.
#[cfg(feature = "ws")]
use okapi::openapi3::Responses;
#[cfg(feature = "ws")]
use rocket::Responder;
#[cfg(feature = "ws")]
use rocket_okapi::response::OpenApiResponder;
#[cfg(feature = "ws")]
use rocket_okapi::util::add_schema_response;

mod event;
pub use event::*;

/// A Rocket responder wrapping String to allow responding with ical data.
#[cfg_attr(feature = "ws", derive(Responder))]
#[cfg_attr(feature = "ws", response(content_type = "text/calendar"))]
pub struct IcalResponder {
	/// The inner String containing ICal data.
	pub inner: String,
}
#[cfg(feature = "ws")]
impl OpenApiResponder<'_> for IcalResponder {
	fn responses(
		gen: &mut rocket_okapi::gen::OpenApiGenerator,
	) -> rocket_okapi::Result<okapi::openapi3::Responses> {
		let mut responses = Responses::default();
		let schema = gen.json_schema::<String>();
		add_schema_response(&mut responses, 200, "text/calendar", schema)?;
		Ok(responses)
	}
}
