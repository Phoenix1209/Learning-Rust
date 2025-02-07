use std::fs::File;
use std::io::ErrorKind;

fn main() {
	let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
		if error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|error| {
				panic!("Problem creating the file: {error:?}", error = error);
			})
		} else {
			panic!("Problem opening the file: {:?}", error);
		}
	});
}

/*
The unwrap method is a shortcut method implemented just like the match expression we wrote in
Listing 9-4. If the Result value is the Ok variant, unwrap will return the value inside the Ok.
If the Result is the Err variant, unwrap will call the panic! macro for us.
*/