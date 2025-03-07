// This code will not compile because the function return a value owned by itself

fn main() {
	let string1 = String::from("abcd");
	let string2 = "xyz";

	let result = longest(string1.as_str(), string2);
	println!("The longest string is {result}");
}

fn longest<'a>(x: &str, y: &str) -> &'a str {
	let result = String::from("really long string");
	result.as_str()
}