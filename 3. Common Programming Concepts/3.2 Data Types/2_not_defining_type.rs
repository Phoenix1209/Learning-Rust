// This code will no compile

#![allow(unused)]
fn main() {
	let guess = "42".parse().expect("Not a number!");
	println!("{guess}")
}