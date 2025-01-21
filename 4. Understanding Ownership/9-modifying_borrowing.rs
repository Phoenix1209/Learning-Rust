// Hay que poner mut en todo sino no compila
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

	println!("value of s: {}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}