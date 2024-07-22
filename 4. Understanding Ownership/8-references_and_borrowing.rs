<<<<<<< HEAD
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s es una referencia a un String
    s.len()
} // Aquí, s sale del alcance. Pero como no tiene propiedad de lo que se refiere,
=======
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s es una referencia a un String
    s.len()
} // Aquí, s sale del alcance. Pero como no tiene propiedad de lo que se refiere,
>>>>>>> 50ab04f68128ae3a1f2ff698801afe071ff2b891
  // no sucede nada.