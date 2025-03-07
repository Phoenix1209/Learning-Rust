/*
'static lifetime denotes that the affected reference can live for the entire
duration of the program.
*/

#![allow(unused)]

fn main() {
	// All string literals have the 'static lifetime, which we can annotate as follows:
	let s: &'static str = "I have a static lifetime.";

	println!("{s}")
}