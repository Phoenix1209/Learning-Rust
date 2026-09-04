// Listing 16-6: Creating a channel and assigning the two halves to tx and rx

// This code does not compile!

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}