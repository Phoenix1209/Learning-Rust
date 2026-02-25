// Listing 8-7: Printing each element in a vector by iterating over the elements using a for loop

fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}