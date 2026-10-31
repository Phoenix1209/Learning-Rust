// Listing 19-21: An unused variable starting with an underscore still binds the value, which might take ownership of the value.

// This code does not compile!

fn main() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{s:?}");
}