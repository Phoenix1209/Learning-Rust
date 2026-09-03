// This code doesn't compile

fn main() {
    let factor = 10;

    fn multiplicar(x: i32) -> i32 {
		x * factor
	} // ERROR

	println!("Resultado: {}", multiplicar(5));
}

/*
	To compile this code we need to define factor variable inside the funcion multiplicar
	or pass it as a parameter to the function.
*/