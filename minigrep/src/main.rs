use std::env;
use std::process;

use minigrep::Config;

fn main() {
	// let args: Vec<String> = env::args().collect;
	let args = env::args().collect::<Vec<_>>();

	let config = Config::build(&args).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {err}");
		process::exit(1);
	});

	eprintln!("[INFO] Searching for {}", &config.query);
	eprintln!("[INFO] In file {}", &config.file_path);
	eprintln!("[INFO] Case Insensitive? {}", config.ignore_case);

	if let Err(e) = minigrep::run(&config) {
		eprintln!("Application error: {e}");
		process::exit(1);
	}
}
