// String Literal: let s = "Hello, world!";
// String Slice: let s = String::from("hello world");

// ¿Qué significa &en las variables?
// Porque imprimí una variable con & al principio y me imprimio el conenido de la variable
// mas no la dirección de memoria.

/* fn main() {
	let mut s = String::from("hello world");

	let word = first_word(&s); // palabra obtendrá el valor 5

	println!("{}, {}", s, word);

	s.clear(); // esto vacía el String, haciéndolo igual a ""

	println!("the first word is: {}", word);

	// la palabra todavía tiene el valor 5 aquí, pero no hay más string
	// con las que podamos usar significativamente el valor 5. ¡la palabra ahora es totalmente inválida!
} */

fn main() {
	let my_string = String::from("hello world");

	// `first_word` works on slices of `String`s, whether partial or whole
	let word = first_word(&my_string[0..6]);
	let word = first_word(&my_string[..]);
	// `first_word` also works on references to `String`s, which are equivalent
	// to whole slices of `String`s
	let word = first_word(&my_string);

	let my_string_literal = "hello world";

	// `first_word` works on slices of string literals, whether partial or whole
	let word = first_word(&my_string_literal[0..6]);
	let word = first_word(&my_string_literal[..]);

	// Because string literals *are* string slices already,
	// this works too, without the slice syntax!
	let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}