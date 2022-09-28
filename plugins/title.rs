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
		let mut title = String::from("Mineral Existence");
		if l.contains("{{title}}") {
			for c in &lines {
				if c.contains("<h2>") {
					title = slice(&c, 4..len(&c) - 5);
					break;
				}
			}
			println!("{}", replace(&l, "{{title}}", &title));
		} else {
			println!("{}", l);
		}
	}
}
