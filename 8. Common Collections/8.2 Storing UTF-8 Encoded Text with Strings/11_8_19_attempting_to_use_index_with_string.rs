// Listing 8-19: Attempting to use indexing syntax with a String

// This code does not compile

fn main() {
    let s1 = String::from("hi");
    let h = s1[0];

	println!("h: {h}");
}