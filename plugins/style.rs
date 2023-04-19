use std::fs;
use std::io;

mod utils;
use utils::text::*;

fn main() {
	let mut lines: Vec<String> = vec![];
	let std_lines = io::stdin().lines();
	for l in std_lines {
		lines.push(l.unwrap());
	}

	for l in &lines {
		if l.contains("{{style}}") {
			// eventually pull the style path from config?
			match fs::read_to_string("public/style.css") {
				Ok(css) => println!("{}", replace(&l, "{{style}}", &css)),
				Err(_) => (),
			}
		} else {
			println!("{}", l);
		}
	}
}
