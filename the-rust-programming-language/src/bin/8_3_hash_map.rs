use std::collections::HashMap;

fn main() {
    basic_operations()
}

#[allow(dead_code)]
fn update_hash_map_values() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{scores:#?}");

    let score = scores.get_mut("Blue");
    match score {
        None => (),
        Some(value) => {
            *value = 30;
        }
    }

    println!("{scores:#?}");
}

#[allow(dead_code)]
fn iterate_over_hash_map() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

#[allow(dead_code)]
fn basic_operations() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Green"), 50);

    // Override entry.
    scores.insert(String::from("Red"), 40);
    // {"Red": 40, "Green": 50}
    println!("{scores:?}");

    // Insert entry if not exist.
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Yellow")).or_insert(150);
    // {"Red": 40, "Yellow": 100, "Green": 50}
    println!("{scores:?}");

    // Update entry if exists.
    scores.entry(String::from("Yellow")).and_modify(|e| {
        *e += 1_000;
    });
    // {"Yellow": 1100, "Red": 40, "Green": 50}
    println!("{scores:?}");
}
