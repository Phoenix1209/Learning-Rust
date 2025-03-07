// This code will not compile because it want to use a variable that is out of scope

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