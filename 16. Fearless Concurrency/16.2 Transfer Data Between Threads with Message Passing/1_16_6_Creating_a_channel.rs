// Listing 16-6: Creating a channel and assigning the two halves to tx and rx

// This code does not compile!

// mpsc stands for multiple producer, single consumer.

// tx = transmitter, rx = receiver

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}