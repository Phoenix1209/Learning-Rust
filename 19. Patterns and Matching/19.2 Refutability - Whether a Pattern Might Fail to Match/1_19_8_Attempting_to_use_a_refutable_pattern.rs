// Listing 19-8: Attempting to use a refutable pattern with let

// This code does not compile!

fn main() {
    let some_option_value: Option<i32> = None;
    let Some(x) = some_option_value;
}