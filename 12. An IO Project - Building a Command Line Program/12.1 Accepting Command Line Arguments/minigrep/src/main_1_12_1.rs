// Listing 12-1: Collecting the command line arguments into a vector and printing them

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	dbg!(args);
}