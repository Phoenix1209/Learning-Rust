// Listing 10-7: The fields x and y must be the same type because both have the same generic data type T.

// This code does not compile because the variables need to be the same type

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
	// One is integer and the another float
    let wont_work = Point { x: 5, y: 4.0 };
}