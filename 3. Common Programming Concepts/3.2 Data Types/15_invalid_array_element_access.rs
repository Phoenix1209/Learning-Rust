//use std::io;
use std::io::{self, Write}; // IMPORTANTE: incluir Write

fn main() {
    let a = [1, 2, 3, 4, 5];
	println!("a: {a:?}");

    print!("\nPlease enter an array index: ");
    io::stdout().flush().unwrap(); // <<< Esto fuerza que se imprima antes de pedir entrada

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}