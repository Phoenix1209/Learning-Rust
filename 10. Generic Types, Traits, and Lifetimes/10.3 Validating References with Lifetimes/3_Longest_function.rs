// This code will not compile because the longest function doesn't
// know the lifetime of the variables

fn main() {
	let string1 = String::from("abcd");
	let string2 = "xyz";

	let result = longest(string1.as_str(), string2);
	println!("The longest string is {result}");
}