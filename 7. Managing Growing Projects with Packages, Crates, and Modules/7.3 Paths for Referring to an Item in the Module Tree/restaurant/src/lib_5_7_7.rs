// Listing 7-7: Adding the pub keyword to mod hosting and fn add_to_waitlist lets us call the
// function from eat_at_restaurant

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// -- snip --
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}