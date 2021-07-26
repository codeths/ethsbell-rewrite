#![cfg(feature = "ws")]

use std::sync::{Arc, Mutex, RwLock};

use chrono::Local;
use ethsbell_rewrite::{schedule::Schedule, SpecLock};
use rocket::{
	http::{ContentType, Status},
	local::Client,
};
use rocket_contrib::templates::Template;

fn client(schedule: Schedule) -> Client {
	let schedule = Arc::new(RwLock::new(schedule));
	let spec_lock = Arc::new(Mutex::new(SpecLock(None)));
	Client::new(
		rocket::ignite()
			.attach(ethsbell_rewrite::api::ApiFairing)
			.attach(ethsbell_rewrite::frontend::FrontendFairing)
			.attach(Template::fairing())
			.manage(schedule.clone())
			.manage(spec_lock),
	)
	.unwrap()
}

#[test]
fn schedule() {
	let mut schedule = Schedule::default();
	schedule.last_updated = Local::now().naive_local();
	let client = client(schedule.clone());
	let mut response = client.get("/api/v1/schedule").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::JSON));
	assert_eq!(
		response.body_string(),
		Some(serde_json::to_string(&schedule).unwrap())
	);
}

#[test]
fn spec() {
	let mut schedule = Schedule::default();
	schedule.last_updated = Local::now().naive_local();
	let client = client(schedule.clone());
	let mut response = client.get("/api/v1/spec").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type(), Some(ContentType::Plain));
	assert_eq!(
		response.body_string(),
		Some(serde_json::to_string(&schedule.definition).unwrap())
	);
}
