// Listing 9-3: Opening a file

use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}