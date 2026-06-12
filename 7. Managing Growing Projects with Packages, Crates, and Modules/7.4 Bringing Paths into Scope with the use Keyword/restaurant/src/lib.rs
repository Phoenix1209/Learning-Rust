// Front of the house section

mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}

		fn seat_at_table() {}
	}

	mod serving {
		fn take_order() {}

		fn serve_order() {}

		fn take_payment() {}
	}
}

// Back of the house section

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

	pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
		// A public function needs to be created to set the value of seasonal_fruit,
		// otherwise an instance of Breakfast could not be created by not giving a value
		// to seasonal_fruit.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

	pub enum Appetizer {
        Soup,
        Salad,
    }

	/* Con los struct se necesita declarar el mismo struct y sus hijos como publicos, en cambio,
	con los enum con solo declarar el enum publico ya todos sus hijos son publicos.	*/
}

// Other

/* Don't write use crate::front_of_house::hosting::add_to_waitlist; because
is unclear where add_to_waitlist is defined */
// use crate::front_of_house::hosting;

// You can use pub too
pub use crate::front_of_house::hosting;

fn deliver_order() {}

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();

	// Using use
	// add_to_waitlist();
	hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod customer {
    pub fn eat_at_restaurant() {
		// If I put hosting::add_to_waitlist(); doesn't work because is out of scope
        super::hosting::add_to_waitlist();
    }
}