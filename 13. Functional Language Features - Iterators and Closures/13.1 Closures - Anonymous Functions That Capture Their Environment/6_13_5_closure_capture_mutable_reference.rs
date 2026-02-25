// Listing 13-5: Defining and calling a closure that captures a mutable reference

fn main() {
	let mut list = vec![1, 2, 3];
	println!("Before defining closure: {list:?}");

	let mut borrows_mutably = || list.push(7);
	// println!("Before calling closure: {list:?}"); // With this it will not compile

	borrows_mutably(); // Need to call variable to push value into vector
	println!("After calling closure: {list:?}");
}