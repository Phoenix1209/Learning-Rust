#![allow(unused)]
fn main() {
	use std::fs;
	use std::io;

	fn read_username_from_file() -> Result<String, io::Error> {
		fs::read_to_string("hello.txt")
	}
}

/*
Reading a file into a string is a fairly common operation, so the standard library provides
the convenient fs::read_to_string function that opens the file, creates a new String, reads
the contents of the file, puts the contents into that String, and returns it. Of course,
using fs::read_to_string doesn’t give us the opportunity to explain all the error handling,
so we did it the longer way first.
*/