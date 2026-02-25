// Listing 7-14: Bringing HashMap into scope in an idiomatic way

use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	map.insert(1, 2);
}