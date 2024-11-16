#![allow(dead_code)]

fn main() {
    unicode();
}

fn mutating_strings_with_plus_operator() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = String::from(" Yooo!");

    // String concatenation requires an owned String, so `s1` had to be cloned (or moved) here.
    let s4 = s1.clone() + &s2 + &s3;

    println!("concatenated string using plus operator: {s4}")
}

fn format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{s1}-{s2}-{s3}");
    println!("concatenated string using format macro: {s4}")
}

fn indexing_strings() {
    let hello_world = String::from("Hello, World!");
    let hello = &hello_world[0..=4];
    println!("hello slice: {hello:#?}");
}

fn unicode() {
    let greating = String::from("cześć");

    // the number of bytes in "cześć" word is: 7
    println!(
        "the number of bytes in \"{greating}\" word is: {}",
        greating.len()
    );

    // the number of characters in "cześć" word is: 5
    println!(
        "the number of characters in \"{greating}\" word is: {}",
        greating.chars().count()
    );
}
