use std::env;
use std::error;
use std::fs;

pub struct Config {
	pub query: String,
	pub file_path: String,
	pub ignore_case: bool,
}

impl Config {
	pub fn build(
		mut args: impl Iterator<Item = String>,
	) -> Result<Config, &'static str> {
		args.next(); // Consume the name of the app.

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string"),
		};

		let file_path = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file path"),
		};

		let ignore_case = env::var("IGNORE_CASE").is_ok();

		Ok(Config {
			query,
			file_path,
			ignore_case,
		})
	}
}

pub fn run(config: &Config) -> Result<(), Box<dyn error::Error>> {
	let contents = fs::read_to_string(&config.file_path)?;

	// let found_lines = if config.ignore_case {
	// 	search_case_insensitive(&config.query, &contents)
	// } else {
	// 	search(&config.query, &contents)
	// };

	let found_lines = match config.ignore_case {
		true => search_case_insensitive(&config.query, &contents),
		false => search(&config.query, &contents),
	};

	for line in found_lines {
		println!("{line}");
	}

	Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(
	query: &str,
	contents: &'a str,
) -> Vec<&'a str> {
	let query = query.to_lowercase();
	contents
		.lines()
		.filter(|l| l.to_lowercase().contains(&query))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_sensitive_no_match() {
		let query = "Pick two";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(Vec::<String>::new(), search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
	}

	#[test]
	fn case_insensitive_no_match() {
		let query = "PICK two";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(
			Vec::<String>::new(),
			search_case_insensitive(query, contents)
		);
	}
}
