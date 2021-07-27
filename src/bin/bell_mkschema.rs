use ethsbell_rewrite::schedule::{
	Event, Period, PeriodType, Schedule, ScheduleDefinition, ScheduleType,
};
use rocket_okapi::JsonSchema;
use schemars::schema_for;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
	std::fs::create_dir_all("./schema").expect("couldn't mkdir");
	mk_schema::<Period>(false);
	mk_schema::<Schedule>(false);
	mk_schema::<ScheduleDefinition>(false);
	mk_schema::<HashMap<String, ScheduleType>>(true);
	mk_schema::<ScheduleType>(false);
	mk_schema::<Event>(false);
	mk_schema::<PeriodType>(false);
}

fn mk_schema<T: JsonSchema>(add_schema_property: bool) {
	let schema = schema_for!(T);
	let mut file = File::create(format!(
		"./schema/{}.json",
		schema.clone().schema.metadata.unwrap().title.unwrap()
	))
	.expect("Couldn't open file");
	let mut schema_json = serde_json::to_value(&schema).unwrap();
	if add_schema_property {
		let object = schema_json.as_object_mut().unwrap();
		match object.get("properties") {
			None => {
				object.insert("properties".to_string(), Value::Object(Map::new()));
			}
			_ => {}
		};
		let properties = object
			.get_mut("properties")
			.unwrap()
			.as_object_mut()
			.unwrap();
		properties.insert(
			"$schema".to_string(),
			Value::Object({
				let mut out = Map::new();
				out.insert("required".to_string(), Value::Array(vec![]));
				out.insert("type".to_string(), Value::String("string".to_string()));
				out
			}),
		);
	}
	write!(
		file,
		"{}",
		serde_json::to_string_pretty(&schema_json).unwrap()
	)
	.unwrap();
}
