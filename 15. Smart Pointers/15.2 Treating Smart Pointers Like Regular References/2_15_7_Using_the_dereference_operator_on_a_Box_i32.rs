fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/*
y es una instancia de Box que apunta a una copia del valor de x,
pero y no es una referencia a x. por lo que hay dos copia de x.
una es la misma x y la otra es la copia de x que se encuentra en el heap y a la que apunta y.

en total serian tres copias del valor 5, una en x, otra en y y otra en el heap (Box) a la que apunta y.
*/