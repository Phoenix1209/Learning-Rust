// This code is an example code and will not compile

impl<T> Option<T> {
	pub fn unwrap_or_else<F>(self, f: F) -> T
	where
		F: FnOnce() -> T
	{
		match self {
			Some(x) => x,
			None => f(),
		}
	}
}

/*
Note: If what we want to do doesn’t require capturing a value from the environment, we can
use the name of a function rather than a closure. For example, we could call
unwrap_or_else(Vec::new) on a Option<Vec<T>> value to get a new, empty vector if the value
is None. The compiler automatically implements whichever of the Fn traits is applicable for a
function definition.
*/