// Listing 10-16: An attempt to use a reference whose value has gone out of scope

// This code does not compile

fn main() {
	let r;

	{
		let x = 5;
		r = &x;
		// This will work
		// r = x;
	}

	println!("r: {r}");
}