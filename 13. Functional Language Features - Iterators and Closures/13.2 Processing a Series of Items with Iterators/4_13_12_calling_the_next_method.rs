// Listing 13-12: Calling the next method on an iterator

fn main () {}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

		// Note that we needed to make v1_iter mutable
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

		/* calling the next method on an iterator changes internal
		state that the iterator uses to keep track of where it
		is in the sequence */
    }
}

/* Also note that the values we get from the calls to next are immutable
references to the values in the vector. The iter method produces an iterator
over immutable references. If we want to create an iterator that takes
ownership of v1 and returns owned values, we can call into_iter instead of
iter. Similarly, if we want to iterate over mutable references, we can call
iter_mut instead of iter. */