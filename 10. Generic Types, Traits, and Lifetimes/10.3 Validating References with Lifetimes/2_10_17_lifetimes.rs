// Listing 10-17: Annotations of the lifetimes of r and x, named 'a and 'b, respectively

// This code does not compile

fn main() {
	let r;                // ---------+-- 'a
						  //          |
	{                     //          |
		let x = 5;        // -+-- 'b  |
		r = &x;           //  |       |
	}                     // -+       |
						  //          |
	println!("r: {r}");   //          |
}                         // ---------+