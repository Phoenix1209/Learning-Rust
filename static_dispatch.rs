// 1. Definimos el comportamiento común
trait Animal {
    fn hacer_sonido(&self);
}

struct Perro;
impl Animal for Perro {
    fn hacer_sonido(&self) {
        println!("Guau");
    }
}

struct Gato;
impl Animal for Gato {
    fn hacer_sonido(&self) {
        println!("Miau");
    }
}

// 2. Función genérica: el tipo T se resuelve al compilar
fn emitir_sonido_estatico<T: Animal>(animal: &T) {
    animal.hacer_sonido();
}

fn main() {
    let perro = Perro;
    let gato = Gato;

    // El compilador genera dos versiones de la función por debajo
    emitir_sonido_estatico(&perro); // Imprime: Guau
    emitir_sonido_estatico(&gato);  // Imprime: Miau
}