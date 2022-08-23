use chrono::{DateTime, Duration, Local};
use ethsbell_rewrite::{aliases::v1::NearbyPeriods, schedule::Period};
use structopt::StructOpt;

/// Macro that prints only if the ugliness value is below the threshold
macro_rules! println_l {
	($q:ident, $t:expr, $($arg:tt)*) => {
		if $q < $t {
			println!($($arg)*);
		}
	};
}

#[derive(StructOpt)]
pub struct Opt {
	/// Set the ETHSBell server URL to use.
	#[structopt(long, default_value = "https://ethsbell.app/")]
	pub server: String,
	/// Set the "fake date" used in the API request when applicable.
	#[structopt(long)]
	pub fake_date: Option<DateTime<Local>>,
	/// Quietness of the output. Accepts up to `-qqqq`, at which point the output will just be unformatted json.
	#[structopt(short, parse(from_occurrences))]
	pub quietness: u64,
}

fn main() {
	let args = Opt::from_args();
	let response: NearbyPeriods = {
		let response_text =
			reqwest::blocking::get(format!("{}/api/v1/today/now/near", args.server))
				.and_then(|v| v.text())
				.expect("Failed to get data from ETHSBell");

		if args.quietness >= 4 {
			println!("{}", response_text);
			return;
		}

		serde_json::from_str(&response_text).expect("Failed to parse ETHSBell data")
	};

	if args.quietness >= 3 {
		println!(
			"{}",
			serde_json::to_string_pretty(&response).expect("failed to beautify")
		);
		return;
	}

	let show_period = |q: u64, n: u8, p: &Period| {
		let period_type = match n {
			0 => "Last",
			1 => "Current",
			2 => "Next",
			_ => unreachable!(),
		};
		let now = Local::now().time();

		println!("=== {} Period ===", period_type);
		println!("Name: {}", p.friendly_name);
		let start_delta = p.start - now;
		if start_delta < Duration::zero() {
			println_l!(q, 2, "Started {} ago", ftime(start_delta))
		} else {
			println_l!(q, 2, "Starts {} from now", ftime(start_delta))
		}
		println_l!(q, 1, "{} long in total", ftime(p.end - p.start));
		println_l!(q, 1, "Period is {:?}", p.kind);

		let end_delta = p.end - now;
		if end_delta < Duration::zero() {
			println_l!(q, 2, "Ended {} ago", ftime(end_delta))
		} else {
			println_l!(q, 2, "Ends {} from now", ftime(end_delta))
		}
		println!()
	};
	let q = args.quietness;
	if let Some(period) = &response.0 {
		show_period(q, 0, period);
	}

	for i in response.1.iter() {
		show_period(q, 1, i);
	}

	if let Some(period) = &response.2 {
		show_period(q, 2, period);
	} else {
		println_l!(q, 2, "=== No Next Period ===")
	}
}

fn ftime(t: Duration) -> String {
	let t = if t < chrono::Duration::zero() {
		t * -1
	} else {
		t
	};
	if t.num_hours().abs() > 0 {
		format!(
			"{}h{}m",
			t.num_hours(),
			(t.num_minutes() - (t.num_hours() * 60))
		)
	} else if (t.num_minutes() - (t.num_hours() * 60)) > 0 {
		format!("{}m{}s", t.num_minutes(), t.num_seconds())
	} else {
		format!(
			"{}s",
			t.num_seconds() - (t.num_minutes() * 60) - (t.num_hours() * 60 * 60)
		)
	}
}
