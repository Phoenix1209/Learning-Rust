<<<<<<< HEAD
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
=======
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
>>>>>>> 50ab04f68128ae3a1f2ff698801afe071ff2b891
// }