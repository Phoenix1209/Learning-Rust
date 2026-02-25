// Listing 8-11: Creating a new, empty String

// use Writte for flush()
use std::io::{stdin, stdout, Write};

fn main() {
    let mut s = String::new();

	print!("Write: ");
    let _=stdout().flush();
	stdin().read_line(&mut s).expect("Failed to read line.");

	print!("{}", s)
}