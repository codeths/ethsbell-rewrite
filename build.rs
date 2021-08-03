use subprocess::Exec;

fn main() {
	println!("cargo:rerun-if-changed=frontend/");
	Exec::shell("npm i").join().unwrap();
	Exec::shell("npm run build").join().unwrap();
}
