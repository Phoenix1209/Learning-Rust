// Listing 4-6: Attempting to modify a borrowed value

// This code does not compile

fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}