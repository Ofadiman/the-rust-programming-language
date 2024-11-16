#![allow(dead_code, unused_variables)]

struct Data {
    value: String,
}

impl Drop for Data {
    // `drop` function will be called automatically when struct goes out of scope, or when manually
    // dropped using `drop(...)` method from standard library.
    fn drop(&mut self) {
        println!("dropping Data struct with value \"{}\"", self.value);
    }
}

fn main() {
    let d1 = Data {
        value: String::from("my stuff"),
    };
    let d2 = Data {
        value: String::from("other stuff"),
    };

    println!("structs created");

    drop(d1);
    drop(d2);
}
