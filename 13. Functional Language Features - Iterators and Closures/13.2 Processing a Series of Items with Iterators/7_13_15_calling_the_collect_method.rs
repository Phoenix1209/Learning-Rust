// Listing 13-15: Calling the map method to create a new iterator and then calling the collect
// method to consume the new iterator and create a vector

fn main() {
	let v1: Vec<i32> = vec![1, 2, 3];

	let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

	assert_eq!(v2, vec![2, 3, 4]);
}

/* You can chain multiple calls to iterator adapters to perform complex actions in a readable
way. But because all iterators are lazy, you have to call one of the consuming adapter methods
to get results from calls to iterator adapters. */