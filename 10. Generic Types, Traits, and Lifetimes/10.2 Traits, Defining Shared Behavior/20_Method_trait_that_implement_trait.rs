fn main() {
	let s = 3.to_string(); // ✅ Funciona porque `i32` implementa `Display`
	println!("{}", s); // Imprime "3"

	let name = "Rust".to_string(); // ✅ Funciona porque `&str` implementa `Display`
	println!("{}", name); // Imprime "Rust"

	// ❌ ERROR: `Vec<i32>` no implementa `Display`, por lo que `to_string()` no está disponible
	// let v = vec![1, 2, 3];
	// let v_string = v.to_string();
	// println!("{}", v_string);

	// ✅ SOLUCIÓN: Convertir `Vec<T>` a `String` de otra forma
	let v = vec![1, 2, 3];
	let v_string = format!("{:?}", v); // Usa `Debug` en lugar de `Display`
	println!("{}", v_string); // Imprime "[1, 2, 3]"
}