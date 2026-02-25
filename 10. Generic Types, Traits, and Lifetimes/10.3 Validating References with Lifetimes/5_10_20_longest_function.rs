// Listing 10-20: An implementation of the longest function that returns the longer of two
// string slices but does not yet compile

// This code does not compile

fn main() {
	let string1 = String::from("abcd");
	let string2 = "xyz";

	let result = longest(string1.as_str(), string2);
	println!("The longest string is {result}");
}

fn longest(x: &str, y: &str) -> &str {
	if x.len() > y.len() { x } else { y }
}