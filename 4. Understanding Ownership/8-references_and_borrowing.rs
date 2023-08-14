fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s es una referencia a un String
    s.len()
} // Aquí, s sale del alcance. Pero como no tiene propiedad de lo que se refiere,
  // no sucede nada.