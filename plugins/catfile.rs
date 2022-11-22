use std::fs;
use std::io;
use std::io::prelude::*;

mod utils;
use utils::text::*;

fn main() {
	for line in io::stdin().lines() {
		let l = line.unwrap();

		if l.contains("{{catfile}}") {
			let mut in_quotes = false;
			let mut path = String::from("");
			for c in l.chars() {
				match c {
					'"' => {
						if in_quotes {
							in_quotes = false;
							path.push(c);
						} else {
							in_quotes = true;
						}
					}
					_ => (),
				}
				if in_quotes {
					path.push(c);
				}
			}
			path = slice(&path, 1..len(&path) - 1);
			// WARNING this shouldn't be hardcoded!
			path = vec!["docs/", &path].concat();

			match fs::read_to_string(&path) {
				Ok(file) => println!("{}", file),
				Err(e) => println!("couldn't find file: {}, failed with error \"{}\"", path, e),
			}
		} else {
			println!("{}", l);
		}
	}
}
