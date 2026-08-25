#[derive(Debug)]

enum List<'a> {
    Cons(i32, &'a List<'a>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let nil = Nil;
    let ten = Cons(10, &nil);
    let a = Cons(5, &ten);
    let b = Cons(3, &a);
    let c = Cons(4, &a);

	println!("{:?}", b);
    println!("{:?}", c);
}