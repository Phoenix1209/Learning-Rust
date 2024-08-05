fn main() {
    let s = String::from("hello");  // s entra en el alcance

    takes_ownership(s);             // s's el valor se mueve a la función ...
                                    // ... y entonces ya no es válido aquí

	// println!("Variable movida: {}", s);	// Error ya que 's' se movió

    let x = 5;                      // x entra en el alcance

    makes_copy(x);                  // x se movería a la función,
                                    // pero i32 es Copy, así que está bien todavía
                                    // use x después
	println!("Variable copiada: {}", x);

} // Aquí, x sale del alcance, luego s. Pero como el valor de s se movió, nada
  // sucede de manera especial.

fn takes_ownership(some_string: String) { // some_string entra en el alcance
    println!("{}", some_string);
} // Aquí, some_string sale del alcance y se llama `drop`. La memoria de
  // respaldo se libera.

fn makes_copy(some_integer: i32) { // some_integer entra en el alcance
    println!("{}", some_integer);
} // Aquí, some_integer sale del alcance. Nada especial sucede.