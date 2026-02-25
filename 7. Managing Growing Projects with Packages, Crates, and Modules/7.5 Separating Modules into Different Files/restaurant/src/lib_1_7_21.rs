// Listing 7-21: Declaring the front_of_house module whose body will be in src/front_of_house.rs

// This code does not compile

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	hosting::add_to_waitlist();
}