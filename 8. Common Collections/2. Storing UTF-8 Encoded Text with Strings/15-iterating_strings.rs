#![allow(unused)]
fn main() {
	for c in "Зд".chars() {
		println!("{c}");
	}

	println!("");

	for b in "Зд".bytes() {
		println!("{b}");
	}

	/*
	Getting grapheme clusters from strings, as with the Devanagari script, is complex, so this
	functionality is not provided by the standard library. Crates are available on crates.io if this
	is the functionality you need.
	*/
}