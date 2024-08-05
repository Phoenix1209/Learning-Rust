fn main() {
    let _s1 = gives_ownership();         // gives_ownership mueve su valor de 
                                        // retorno a s1

    let s2 = String::from("hello");     // s2 entra en el alcance

    let _s3 = takes_and_gives_back(s2);  // s2 se mueve a takes_and_gives_back, 
                                        // que también mueve su valor 
                                        // de retorno a s3
} // Aquí, s3 sale del alcance y se descarta. s2 sale del alcance pero se movió,
  // por lo que no sucede nada. s1 sale del alcance y se descarta.

fn gives_ownership() -> String {             // gives_ownership moverá su 
                                             // valor de retorno a la función 
                                             // que lo llama.

    let some_string = String::from("hello"); // some_string entra en el alcance

    some_string                              // some_string se devuelve y 
                                             // se mueve a la función de llamada.
}

// takes_and_gives_back tomará un *string* y devolverá uno
fn takes_and_gives_back(a_string: String) -> String { // a_string entra en el alcance

    a_string  // a_string se devuelve y se mueve a la función de llamada
}