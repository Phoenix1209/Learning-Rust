// Listing 5-5: A build_user function that uses field init shorthand because the username and
// email parameters have the same name as struct fields

struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}

fn main() {
	let user1 = build_user(
		String::from("someone@example.com"),
		String::from("someusername123"),
	);
}