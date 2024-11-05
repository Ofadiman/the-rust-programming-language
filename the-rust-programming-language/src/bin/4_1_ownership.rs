// Ownership rules:
// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

fn main() {
    function_returns_ownership()
}

#[allow(dead_code)]
fn string() {
    // Strings are mutable and are allocated on the heap.
    let mut string = String::from("Hello");
    string.push_str(", World!");
    println!("{string}");
}

#[allow(dead_code)]
fn string_literal() {
    // String literals are immutable and are allocated on the stack.
    let literal = "immutable";
    println!("{literal}")
}

#[allow(dead_code)]
fn print_pointer() {
    let pointer: &String = &String::from("pointer");
    // Rust is dereferencing variables by default, so to print pointers we have to use appropriate
    // formatter.
    println!("{pointer:p}");
}

#[allow(dead_code)]
fn string_capacity() {
    // Allocate 32 bytes of capacity by default.
    let mut capacity: String = String::with_capacity(32);

    // capacity: 32, memory address: 0x7ffda82941c8
    println!(
        "capacity: {}, memory address: {:p}",
        capacity.capacity(),
        &capacity
    );

    // capacity: 64, memory address: 0x7ffda82941c8
    capacity.push_str(&"x".repeat(33));
    println!(
        "capacity: {}, memory address: {:p}",
        capacity.capacity(),
        &capacity
    );
}

#[allow(dead_code)]
fn pointers_after_reassing() {
    // pointer: 0x7fff989c06d8
    let s1 = String::from("Hello, World!");
    println!("pointer: {:p}", &s1);

    // pointer: 0x7fff989c0748
    let s2 = s1;
    println!("pointer: {:p}", &s2);
}

#[allow(dead_code)]
fn cloning() {
    let s1 = String::from("Hello, World!");
    let s2 = s1.clone();

    // No error.
    println!("{s1}, {s2}");

    let x = 5;
    let y = x;

    // No error. Data on the stack is cloned automatically.
    println!("x = {x}, y = {y}");
}

#[allow(dead_code)]
fn functions_taking_ownership() {
    fn function(s: String) {
        println!("{s}")
    }

    let string = String::from("Hello, World!");

    // `function` takes ownership of `string`.
    function(string);
    // `string` moved to `function` and is no longer valid here.
}

#[allow(dead_code)]
fn functions_making_copy() {
    fn function(i: i32) {
        println!("{i}");
    }

    let int = 5;

    // `function` makes copy of `int` because `int` is a simple type stored on the stack.
    function(int);

    // `int` is still valid here.
    println!("{int}");
}

#[allow(dead_code)]
fn function_returns_ownership() {
    fn function(string: String) -> String {
        format!("{string}, World!")
    }

    let string = String::from("Hello");

    // `function` takes ownership of `string` and returns ownership of a String.
    let mutated_string = function(string);
    // `string` moved to `function` and is no longer valid here.

    println!("{mutated_string}");
}
