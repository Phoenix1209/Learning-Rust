// Listing 16-5: Using the move keyword to force a closure to take ownership of the values it uses

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

	// can't use because handle has ownership of v
    // drop(v); // oh no!

    handle.join().unwrap();

	// even here the main thread can't use v
    // drop(v); // oh no!
}