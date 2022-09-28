pub mod text {

	use std::ops::Range;

	/*
	returns the length of a String, taking graphemes into account
	*/
	pub fn len(s: &String) -> usize {
		s.chars().count()
	}

	/*
	replaces all target str in String with insert str
	*/
	pub fn replace(s: &String, target: &str, insert: &str) -> String {
		let mut source = s.clone();
		while let Some(i) = source.find(target) {
			source.replace_range(i..i + len(&target.to_string()), insert);
		}
		source
	}

	/*
	returns a slice of a string from a range, utf-8 compliant
	*/
	pub fn slice(s: &String, r: Range<usize>) -> String {
		let begin = s.char_indices().nth(r.start).map(|(i, _)| i).unwrap_or(0);
		let end = s
			.char_indices()
			.nth(r.end)
			.map(|(i, _)| i)
			.unwrap_or(s.len());
		s[begin..end].to_string()
	}
}
