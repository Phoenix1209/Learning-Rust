fn main() {
    let factor = 10;

    // Esto NO compilaría como función normal, porque `fn` no puede
    // capturar `factor` del entorno:
    let multiplicar = |x: i32| x * factor; // OK, closure captura `factor`

    println!("{}", multiplicar(5)); // 50
}