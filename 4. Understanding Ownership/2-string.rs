fn main() {
	let mut s = String::from("hello");

	s.push_str(", world!"); // push_str() agrega un literal a un String

	println!("{}", s);      // Esto imprimirá `hello, world!`
}