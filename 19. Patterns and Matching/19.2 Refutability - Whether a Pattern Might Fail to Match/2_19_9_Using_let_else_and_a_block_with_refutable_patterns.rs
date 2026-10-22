// Listing 19-9: Using let...else and a block with refutable patterns instead of let

fn main() {
    let some_option_value: Option<i32> = None;
    let Some(x) = some_option_value else {
        return;
    };
}