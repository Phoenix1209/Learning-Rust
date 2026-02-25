// Listing 7-3: Calling the add_to_waitlist function using absolute and relative paths

// This code does not compile

mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {}
	}
}

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();
}