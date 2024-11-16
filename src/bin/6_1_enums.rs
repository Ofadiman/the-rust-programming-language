#![allow(dead_code)]

use rand::Rng;

fn main() {
    results()
}

fn extract_values_from_enum() {
    #[derive(Debug)]
    enum Color {
        Rgb(u8, u8, u8),
        Hex(String),
    }

    let number = rand::thread_rng().gen_range(1..=2);
    let color = if number == 1 {
        Color::Rgb(252, 186, 3)
    } else {
        Color::Hex("#fcba03".to_string())
    };

    let computed: String = match color {
        Color::Rgb(red, green, blue) => format!("red: {red}, green: {green}, blue: {blue}"),
        Color::Hex(value) => format!("hex: {value}"),
    };

    println!("{computed:#?}");
}

fn results() {
    let number = rand::thread_rng().gen_range(1..=2);
    let option = if number == 1 { Some(1) } else { None };

    if let Some(value) = option {
        println!("value from enum: {value}");
    } else {
        println!("nothing here");
    }
}
