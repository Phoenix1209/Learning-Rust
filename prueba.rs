// To flush input
use std::io::{self, Write};

fn main(){
	let mut var1 = String::new();

	print!("Ingrese palabras con espacios: ");
	let _ = io::stdout().flush();
	std::io::stdin().read_line(&mut var1).expect("Error al leer línea.");

	let var2 = var1.trim();

	println!("var1: {var1}");
	println!("var2: {var2}");
}