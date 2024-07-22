<<<<<<< HEAD
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

/*	No se puede tener una referencia inmutable mientras se tenga una referencia mutable
	ya que los usuarios que estarían leyendo la variable verían que va cambiando y no
=======
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

/*	No se puede tener una referencia inmutable mientras se tenga una referencia mutable
	ya que los usuarios que estarían leyendo la variable verían que va cambiando y no
>>>>>>> 50ab04f68128ae3a1f2ff698801afe071ff2b891
	queremos eso. */