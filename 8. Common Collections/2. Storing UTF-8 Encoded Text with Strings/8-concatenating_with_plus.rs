fn main() {
	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
	// let s3 = &s1 + &s2;	// Rust forces you to take ownership of the first string and borrow the second

	// println!("{s1}");
	println!("{s2}");
	println!("{s3}");
}

/*
The + operator uses the add method, whose signature looks something like this:
fn add(self, s: &str) -> String {} 
*/