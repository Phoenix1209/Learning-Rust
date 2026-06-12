fn main() {
	let string1 = String::from("abcd");
	let result;
	{
	let string2 = "xyz";
	//result = longest(string1.as_str(), string2).to_string();
	//result = String::from(longest(string1.as_str(), string2));
	// .to_owned() hace una copia innecesaria en memoria, como to_string().
	result = longest(string1.as_str(), string2).to_owned();
	}
	println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}