use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

fn main() {
    loop_over_collections()
}

#[allow(dead_code)]
fn if_else_if_else() {
    let mut rng: Pcg64 = Seeder::from("stripy zebra").make_rng();

    let number: u32 = rng.gen();

    if number == 5 {
        println!("got 5")
    } else if number == 3 {
        println!("got 3")
    } else {
        println!("got {number}")
    }
}

#[allow(dead_code)]
fn if_in_let() {
    let mut rng = StdRng::seed_from_u64(1);
    let condition: bool = rng.gen();
    let number = if condition { 5 } else { 3 };
    println!("condition: {condition}, number: {number}")
}

#[allow(dead_code)]
fn looping() {
    let mut counter = 0;
    loop {
        if counter == 10 {
            break;
        }

        println!("counter: {counter}");
        counter = counter + 1;
    }
}

#[allow(dead_code)]
fn loop_labels() {
    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;

        'counting_down: loop {
            if count == 2 {
                break 'counting_up;
            }

            if remaining == 8 {
                break 'counting_down;
            }

            remaining -= 1;

            println!("count = {count}, remaining = {remaining}");
        }

        count += 1;
    }
}

#[allow(dead_code)]
fn loop_over_collections() {
    let vector = vec![1, 2, 3, 4, 5];
    for number in vector {
        println!("from vector: {number}");
    }

    let array = ["one".to_string(), "two".to_string(), "three".to_string()];
    for number in array {
        println!("from array: {number}")
    }

    for number in (1..=5).rev() {
        println!("from range: {number}")
    }
}
