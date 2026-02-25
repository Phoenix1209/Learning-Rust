// Listing 9-10: Attempting to use the ? in the main function that returns () won’t compile.

// This program does not compile

use std::fs::File;

fn main() {
	let greeting_file = File::open("hello.txt")?;
}