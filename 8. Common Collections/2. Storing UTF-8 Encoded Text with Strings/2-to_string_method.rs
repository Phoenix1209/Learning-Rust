use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let data = "initial contents";

    println!("data type: {}", type_of(&data));

    let s = data.to_string();
    println!("s type: {}", type_of(&s));

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}