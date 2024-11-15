#![allow(dead_code)]

use std::thread;
use std::time::Duration;

fn main() {
    closures_in_results()
}

fn closures_in_results() {
    let result: Result<String, String> = Err("failed".to_string());

    let error_message: String = result.unwrap_or_else(|message| message);

    println!("error message: {error_message}")
}

fn closure_definition() {
    // Closures can be called like regular functions.
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(3);
}

fn closure_borrowing() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn closure_borrowing_mutably() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}

fn closure_moving() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
