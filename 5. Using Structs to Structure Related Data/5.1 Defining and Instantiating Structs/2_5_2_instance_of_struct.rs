// Listing 5-2: Creating an instance of the User struct

struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn main() {
	let user1 = User {
		active: true,
		username: String::from("someusername123"),
		email: String::from("someone@example.com"),
		sign_in_count: 1,
	};
}