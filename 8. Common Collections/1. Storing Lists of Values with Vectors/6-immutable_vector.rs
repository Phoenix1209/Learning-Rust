// No compila
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

	// aquí está el error porque ya no se puede usar v
    v.push(6);

    println!("The first element is: {first}");
}