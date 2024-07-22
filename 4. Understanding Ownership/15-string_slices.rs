<<<<<<< HEAD
fn main() {
	let s = String::from("hello");

	let slice = &s[0..2];
	println!("{}",slice);
	let slice = &s[..2];
	println!("{}",slice);

	let len = s.len();

	let slice = &s[3..len];
	println!("{}",slice);
	let slice = &s[3..];
	println!("{}",slice);

	let slice = &s[0..len];
	println!("{}",slice);
	let slice = &s[..];
	println!("{}",slice);
=======
fn main() {
	let s = String::from("hello");

	let slice = &s[0..2];
	println!("{}",slice);
	let slice = &s[..2];
	println!("{}",slice);

	let len = s.len();

	let slice = &s[3..len];
	println!("{}",slice);
	let slice = &s[3..];
	println!("{}",slice);

	let slice = &s[0..len];
	println!("{}",slice);
	let slice = &s[..];
	println!("{}",slice);
>>>>>>> 50ab04f68128ae3a1f2ff698801afe071ff2b891
}