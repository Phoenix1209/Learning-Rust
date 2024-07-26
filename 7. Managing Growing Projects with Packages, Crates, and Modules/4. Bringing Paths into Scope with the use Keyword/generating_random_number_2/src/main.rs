/* If we’re using multiple items defined in the same crate or same module,
listing each item on its own line can take up a lot of vertical space in our files. */

use rand::Rng;
/* // --snip--
use std::cmp::Ordering;
use std::io;
// --snip-- */

// Instead, we can use nested paths to bring the same items into scope in one line.
// --snip--
use std::{cmp::Ordering, io};
// --snip--

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

	let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}