// Listing 10-6: A Point<T> struct that holds x and y values of type T

struct Point<T> {
	x: T,
	y: T,
}

fn main() {
	let integer = Point { x: 5, y: 10 };
	let float = Point { x: 1.0, y: 4.0 };
}