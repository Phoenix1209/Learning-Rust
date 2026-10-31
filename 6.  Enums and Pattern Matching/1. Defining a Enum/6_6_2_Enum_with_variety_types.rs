// Listing 6-2: A Message enum whose variants each store different amounts and types of values

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {}