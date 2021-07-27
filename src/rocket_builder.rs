use crate::{api, frontend, schedule::Schedule, SpecLock};
use rocket::Rocket;
use rocket_contrib::templates::Template;
use rocket_prometheus::PrometheusMetrics;
use std::sync::{Arc, Mutex, RwLock};

// this file just includes the function to build rocket

pub fn rocket(schedule: Schedule) -> Rocket {
	let schedule = Arc::new(RwLock::new(schedule));
	let spec_lock = Arc::new(Mutex::new(SpecLock(None)));
	let prometheus = PrometheusMetrics::new();
	rocket::ignite()
		.attach(prometheus.clone())
		.mount("/metrics", prometheus)
		.attach(api::ApiFairing)
		.attach(frontend::FrontendFairing)
		.attach(Template::fairing())
		.manage(schedule.clone())
		.manage(spec_lock)
}
