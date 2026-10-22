// Listing 19-2: Incorrectly constructing a pattern whose variables don’t match the number of elements in the tuple

// This code does not compile!

fn main() {
    let (x, y) = (1, 2, 3);
}