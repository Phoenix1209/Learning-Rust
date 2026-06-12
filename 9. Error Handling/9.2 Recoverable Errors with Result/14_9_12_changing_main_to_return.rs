// Listing 9-12: Changing main to return Result<(), E> allows the use of the ? operator on Result values.

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
	let greeting_file = File::open("hello.txt")?;

	Ok(())
}