use std::fmt::Display;

struct Pair<T> {
	x: T,
	y: T,
}

// Implementación sin restricciones: Funciona para cualquier tipo `T`
impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

// Implementación condicional: Solo para `T` que implementa `Display + PartialOrd`
impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}

fn main() {
	let pair1 = Pair::new(3, 5);
	pair1.cmp_display(); // ✅ Funciona: `i32` implementa `Display` y `PartialOrd`

	let pair2 = Pair::new("apple", "banana");
	pair2.cmp_display(); // ✅ Funciona: `&str` implementa `Display` y `PartialOrd`

	// ❌ ERROR: `Vec<i32>` NO implementa `PartialOrd`, por lo que `cmp_display` no está disponible
	// let pair3 = Pair::new(vec![1, 2], vec![3, 4]);
	// pair3.cmp_display();

	// ✅ PERO: Se puede usar `new` sin problemas, ya que no tiene restricciones
	let pair4 = Pair::new(vec![1, 2], vec![3, 4]);
	// pair4.cmp_display(); // ❌ ERROR: Método no disponible en `Vec<i32>`
}	