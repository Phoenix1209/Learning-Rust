// This program will not compile

use std::fs::File;

fn main() {
	let greeting_file = File::open("hello.txt")?;
}