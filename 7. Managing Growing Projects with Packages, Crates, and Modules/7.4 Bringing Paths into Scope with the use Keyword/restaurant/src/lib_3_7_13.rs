// Listing 7-13: Bringing the add_to_waitlist function into scope with use, which is unidiomatic

mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
	add_to_waitlist();
}