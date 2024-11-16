#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    matching_on_errors();

    return Ok(());
}

fn matching_on_errors() {
    let greeting_file_result = File::open("hello.txt");

    match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

fn propagating_errors() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        // Question mark operator can be used on `Result` type to extract `Ok` value, and if error
        // occurred, propagate the error to the caller.
        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // Question mark operator can also be used on `Option` type to extract `Some` value, or to
    // propagate `None` value to the caller.
    text.lines().next()?.chars().last()
}
