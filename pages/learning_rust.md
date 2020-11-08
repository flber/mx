### Learning Rust

First of all, this is going to primarily be a place to store notes for myself as I learn Rust, so take everything I say with a grain of salt! (and feedback is always appreciated, so feel free to say hi)

Good resources:
- [[Simple Rust GUI with QML]](https://www.vandenoever.info/blog/2017/02/17/a-simple-rust-gui-with-qml.html)
- [[Rust Programming Tutorial]](https://www.youtube.com/playlist?list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL)
- [[The Rust Programming Language]](https://doc.rust-lang.org/book)

Things to remember:
- variable(y) things
	- variables need the `mut` tag to be changeable
	- it's statically typed! (but has inference)
	- tuples can hold different types, arrays cannot
		- values in tuples can be accessed through `tuple.index`
	- vectors are like arrays, but they can change size
	- arrays can be defined as `let arr = [3; 4];`, which would expand to `[3, 3, 3, 3]`
	- `&` means a reference to an object?
	- references are immutable by default
	- enums have a fixed set of variables, called the variants
	- can reassign variables, overwriting previous type and value (good for converting between types), called shadowing
	- always have to annotate type for constants
- style things
	- most names should be in `snake_case`
	- types, enums, and traits should be in `CamelCase`
	- anything static/constant should be in `SCREAMING_SNAKE_CASE`
	- use underscores in big numbers to break them up (`10_000_000`)
- various functions/expression things
	- `.expect()` is used to handle possible errors
	- the `match` expression
		- is made of arms
		- each arm has a pattern and some code to run
		- connect the code to the pattern with `=>`
		- separate the arms with commas, not semicolons
		- can also use `match` instead of a `.expect()` to handle errors (match output to `Ok` and `Err`)
- miscellaneous things
	- if it looks like a function but has a `!` at the end, it's a macro (?? what's a macro ??)
	- `::` refers to an associated function of a "type" (static method)
	- the `Result` enum has two variants, `Ok` and `Err`
	- `gen_range()` is inclusive on the lower bound and exclusive on the upper
	- can use `cargo doc --open` to view the documentation of the packages in your project

Things I'm not sure about
- macros
- passing objects by reference
- `#[]` <- these things
- what is a type?
- traits
