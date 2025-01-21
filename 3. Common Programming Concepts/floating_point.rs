use std::any::type_name;

// La única forma de imprimir el tipo de dato de una variable en Rust es por medio de esta función
fn print_type_of<T>(_: &T) {
    println!("El tipo de dato es: {}", type_name::<T>());
}

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    print_type_of(&x); // Imprime: El tipo de dato es: f64
    print_type_of(&y); // Imprime: El tipo de dato es: f32
}