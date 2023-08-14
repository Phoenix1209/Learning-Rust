fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle devuelve una referencia a un String

    let s = String::from("hello"); // s es una nuevo String

    &s // devolvemos una referencia a String, s
} // ¡Toma! s sale del alcance y se descarta. Su memoria desaparece.
  // ¡Peligro!

// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }