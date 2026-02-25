// Listing 13-14: Calling the iterator adapter map to create a new iterator

fn main() {
	let v1: Vec<i32> = vec![1, 2, 3];

	v1.iter().map(|x| x + 1);
}