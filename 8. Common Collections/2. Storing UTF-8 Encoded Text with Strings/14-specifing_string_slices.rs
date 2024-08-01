#![allow(unused)]
fn main() {
	let hello = "Здравствуйте";

	// If I put [0..1] it will panic
	let s = &hello[0..4];

	println!("{s}")
}