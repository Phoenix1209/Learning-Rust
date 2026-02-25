// Listing 5-7: Using struct update syntax to set a new email value for a User instance but
// to use the rest of the values from user1

struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn main() {
	// --snip--

	let user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	let user2 = User {
		email: String::from("another@example.com"),
		..user1
	};
}