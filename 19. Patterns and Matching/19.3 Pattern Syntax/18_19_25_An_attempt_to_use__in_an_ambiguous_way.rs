// Listing 19-25: An attempt to use .. in an ambiguous way

// This code does not compile!

fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {second}")
        },
    }
}