fn main() {
	// String (owned, modificable)
	let mut s1 = String::from("Hola");
	s1.push_str(" mundo"); // Esto es posible con String

	// &str (referencia inmutable)
	let mut s2: &str = "Hola"; 
	s2.push_str(" mundo"); // Esto causaría un error de compilación
}