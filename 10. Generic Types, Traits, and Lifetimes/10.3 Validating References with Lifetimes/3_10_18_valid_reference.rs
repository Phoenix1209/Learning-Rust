// Listing 10-18: A valid reference because the data has a longer lifetime than the reference

fn main() {
	let x = 5;            // ----------+-- 'b
						  //           |
	let r = &x;           // --+-- 'a  |
						  //   |       |
	println!("r: {r}");   //   |       |
						  // --+       |
}                         // ----------+