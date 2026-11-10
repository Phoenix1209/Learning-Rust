// Reutilizamos el trait Animal, Perro y Gato del ejemplo anterior

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

// 2. Función que recibe un Trait Object (&dyn Animal)
fn emitir_sonido_dinamico(animal: &dyn Animal) {
    animal.hacer_sonido();
}

fn main() {
    // Podemos mezclar Perro y Gato en el mismo vector usando Box<dyn Animal>
    let mascotas: Vec<Box<dyn Animal>> = vec![
        Box::new(Perro),
        Box::new(Gato),
    ];

    // El programa decide qué método llamar en tiempo de ejecución
    for mascota in mascotas {
        emitir_sonido_dinamico(mascota.as_ref());
    }
}