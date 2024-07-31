fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");

        // Error due to borrowing
        v.push(6);
    }
}