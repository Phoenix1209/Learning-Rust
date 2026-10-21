// Listing 18-10: Attempting to use a type that doesn’t implement the trait object’s trait

// This code does not compile!

use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}