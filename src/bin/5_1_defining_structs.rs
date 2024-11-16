#![allow(dead_code)]

fn main() {
    tuples();
}

fn struct_update_syntax() {
    #[derive(Debug, Clone)]
    struct User {
        id: u64,
        first_name: String,
        last_name: String,
    }
    let john = User {
        id: 1,
        first_name: "john".to_string(),
        last_name: "doe".to_string(),
    };

    let johns_brother = User {
        id: 2,
        first_name: "mark".to_string(),
        // Fill the rest of the fields using fields from passed struct.
        ..john.clone()
    };

    println!("{john:#?}");
    println!("{johns_brother:#?}");
}

fn tuples() {
    #[derive(Debug)]
    struct Color(u8, u8, u8);

    let red = Color(255, 0, 0);

    println!("{red:#?}");
    println!("r: {}, g: {}, b: {}", red.0, red.1, red.0)
}
