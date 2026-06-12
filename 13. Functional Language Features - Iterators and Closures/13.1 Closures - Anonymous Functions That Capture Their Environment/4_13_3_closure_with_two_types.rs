// Listing 13-3: Attempting to call a closure whose types are inferred with two different types
// This code will not compile

fn main() {
	let example_closure = |x| x;

	let s = example_closure(String::from("hello"));
	let n = example_closure(5);
}