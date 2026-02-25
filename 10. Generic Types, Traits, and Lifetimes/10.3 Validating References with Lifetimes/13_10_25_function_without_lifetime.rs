// Listing 10-25: A function we defined in Listing 4-9 that compiled without lifetime annotations,
// even though the parameter and return type are references

/* The patterns programmed into Rust’s analysis of references are called the
lifetime elision rules.

This code can compile without lifetime reference because of the pattern
fn first_word<'a>(s: &'a str) -> &'a str {} */

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	println!("as_bytes: {:?}", bytes);

	for (i, &item) in bytes.iter().enumerate() {
		println!("i: {}", i);
		println!("item: {}", item);
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

fn main() {
	let my_string = String::from("hello world");

	// first_word works on slices of `String`s
	let word = first_word(&my_string[..]);
	println!("word: {}", word);

	let my_string_literal = "hello world";
	println!("my_string_literal: {}", my_string_literal);

	// first_word works on slices of string literals
	let word = first_word(&my_string_literal[..]);
	println!("word: {}", word);

	// Because string literals *are* string slices already,
	// this works too, without the slice syntax!
	let word = first_word(my_string_literal);
	println!("word: {}", word);
}