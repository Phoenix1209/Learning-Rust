// Listing 13-13: Calling the sum method to get the total of all items in the iterator

fn main () {}

#[cfg(test)]
mod tests {
	#[test]
	fn iterator_sum() {
		let v1 = vec![1, 2, 3];

		let v1_iter = v1.iter();

		let total: i32 = v1_iter.sum();

		assert_eq!(total, 6);
	}
}

/* We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the
iterator we call it on. */