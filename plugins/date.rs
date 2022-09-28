use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};

mod utils;
use utils::text::*;

fn main() {
	let process = match Command::new("date")
		.arg("+%m%d%y")
		.stdout(Stdio::piped())
		.spawn()
	{
		Err(e) => panic!("couldn't spawn process: {}", e),
		Ok(process) => process,
	};
	let mut date = String::new();
	match process.stdout.unwrap().read_to_string(&mut date) {
		Err(e) => panic!("couldn't read script stdout: {}", e),
		Ok(_) => (),
	}
	date = slice(&date, 0..len(&date) - 1);

	for line in io::stdin().lines() {
		let l = line.unwrap();

		if l.contains("{{date}}") {
			println!("{}", replace(&l, "{{date}}", &date));
		} else {
			println!("{}", l);
		}
	}
}
