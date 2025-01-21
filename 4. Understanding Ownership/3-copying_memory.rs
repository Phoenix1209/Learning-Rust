fn main() {
	let s1 = String::from("hello");
	let s2 = s1.clone(); // Si no se pone .clone() el programa no compila

	println!("{}, world!", s1);
	println!("{}, world!", s2);
}