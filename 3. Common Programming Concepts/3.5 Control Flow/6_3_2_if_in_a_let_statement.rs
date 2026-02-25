// Listing 3-2: Assigning the result of an if expression to a variable

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}