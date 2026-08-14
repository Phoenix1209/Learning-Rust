use std::fmt::Display;

// Implementamos `ToString` para cualquier tipo `T` que implemente `Display`
impl<T: Display> ToString for T {
	// Implementación automática de `to_string()` para `T`
}

fn main() {
	let num = 3;
	println!("{}", num.to_string()); // ✅ Funciona porque `i32` implementa `Display`

	let float = 3.14;
	println!("{}", float.to_string()); // ✅ Funciona porque `f64` implementa `Display`

	let text = "Rust";
	println!("{}", text.to_string()); // ✅ Funciona porque `&str` implementa `Display`

	// ❌ ERROR: `Vec<i32>` NO implementa `Display`, por lo que `to_string()` no está disponible
	// let numbers = vec![1, 2, 3];
	// println!("{}", numbers.to_string());

	// SOLUCIÓN: Usar `format!("{:?}", variable)` para tipos que no implementan `Display`
	let numbers = vec![1, 2, 3];
	println!("{}", format!("{:?}", numbers)); // ✅ Funciona: Usa `Debug` en lugar de `Display`
}