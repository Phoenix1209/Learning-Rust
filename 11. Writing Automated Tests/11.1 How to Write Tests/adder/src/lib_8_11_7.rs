// Listing 11-7: Testing the function add_two using the assert_eq! macro

pub fn add_two(a: usize) -> usize {
	a + 2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_adds_two() {
		let result = add_two(2);
		assert_eq!(result, 4);
	}
}