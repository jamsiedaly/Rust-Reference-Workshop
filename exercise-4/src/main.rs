use std::fmt::{Debug};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}

