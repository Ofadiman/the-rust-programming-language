fn main() {
    vector_of_unions()
}

#[allow(dead_code)]
fn basic_vector_operations() {
    let mut numbers = vec![0, 1, 2, 3, 4];

    let third = numbers.get(3);
    match third {
        Some(number) => println!("The third element is {number}"),
        None => println!("There is no third element."),
    }

    let second = numbers.get_mut(2);
    match second {
        Some(number) => {
            // Update number using dereferencing.
            *number = 10;
        }
        None => println!("There is no third element."),
    }
    println!("{numbers:#?}");

    // Iterate over mutable reference to numbers updating their values in memory.
    for number in &mut numbers {
        *number += 100;
    }
    println!("{numbers:#?}");
}

#[allow(dead_code)]
fn vector_of_unions() {
    #[derive(Debug)]

    enum Color {
        Rgb(u8, u8, u8),
        Hex(String),
    }

    // Rust does not have "union" type from TypeScript. It instead uses enums to represent multiple
    // values in a vector.
    let colors = vec![Color::Rgb(0, 0, 0), Color::Hex("#4287f5".to_string())];
    for color in colors {
        match color {
            Color::Rgb(_, _, _) => {
                println!("working with rgb color")
            }
            Color::Hex(_) => {
                println!("working with hex color")
            }
        }
    }
}
