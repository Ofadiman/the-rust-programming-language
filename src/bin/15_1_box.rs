#![allow(dead_code, unused_variables)]

// Cons list is a recursive data structure from Lisp programming language.
enum List {
    // Box type allows to create recursive types by storing the value on the heap and pointer to
    // the value on the stack.
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    match list {
        List::Nil => println!("got nil"),
        List::Cons(integer, list) => {
            println!("got integer: {integer}");
            match *list {
                List::Nil => println!("got nil"),
                List::Cons(integer, list) => {
                    println!("got integer: {integer}");
                    match *list {
                        List::Nil => println!("got nil"),
                        List::Cons(integer, list) => {
                            println!("got integer: {integer}");
                            match *list {
                                List::Nil => println!("got nil"),
                                List::Cons(integer, list) => {
                                    println!("integer: {integer}")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
