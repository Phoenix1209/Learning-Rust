fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // I added the mut word
    let mut first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
}