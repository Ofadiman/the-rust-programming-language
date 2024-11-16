#![allow(dead_code)]

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// Rc type keeps track of the number of references to a value to determine whether or not the value
// is still in use. If there are 0 references to a value, the value can be dropped safely. This
// method allows to have many owners of 1 value.
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // current reference count = 1
    println!("current reference count = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    // current reference count = 2
    println!("current reference count = {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    // current reference count = 3
    println!("current reference count = {}", Rc::strong_count(&a));

    drop(b);
    // current reference count = 2
    println!("current reference count = {}", Rc::strong_count(&a));

    drop(c);
    // current reference count = 1
    println!("current reference count = {}", Rc::strong_count(&a));
}
