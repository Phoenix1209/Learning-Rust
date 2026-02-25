// Listing 3-3: Using a while loop to run code while a condition evaluates to true

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}