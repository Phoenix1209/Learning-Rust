use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let root = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Rc::downgrade(&root)),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *root.children.borrow_mut() = vec![Rc::clone(&branch)]; // branch apunta a root como padre

	/*
	Nota interesante: tu línea *root.children.borrow_mut() = vec![Rc::clone(&branch)];
	reemplaza todo el Vec en vez de hacer push. Funciona bien aquí porque root.children
	estaba vacío, pero si root ya tuviera hijos antes de esa línea, los perderías todos. Es la
	diferencia entre "asignar" (*x.borrow_mut() = nuevo_valor) y "modificar en sitio"
	(x.borrow_mut().push(...)) — ambas son válidas, pero hacen cosas distintas.
	*/

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // leaf apunta a branch como padre

	println!();

    println!(
        "root strong = {}, weak = {}",
        Rc::strong_count(&root),
        Rc::weak_count(&root),
    );

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

	println!();

    println!(
        // "branch parent value = {:#?}",
        // branch.parent.borrow().upgrade()
        "branch parent value = {:?}",
		branch.parent.borrow().upgrade().map(|n| n.value)
    );

    println!(
        "branch all children values = {:?}",
	    branch.children.borrow().iter().map(|n| n.value).collect::<Vec<i32>>()
    );

	println!();
}