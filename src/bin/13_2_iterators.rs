#![allow(dead_code)]

fn main() {
    functional_programming()
}

fn implicit_iterators() {
    let mut numbers = vec![1, 2, 3];

    // For loop creates iterators implicitly and takes ownership of the data, so `numbers` vector
    // will be moved if I don't use reference here.
    // reference,
    for number in &numbers {
        dbg!(number);
    }

    for value in &mut numbers {
        *value = *value * 2;
    }

    dbg!(numbers);
}

// Methods from lodash/ramda are available on iterators, not vectors.
fn functional_programming() {
    let numbers = vec![1, 2, 3];

    // `iter` method produces iterator which does not take ownership of `numbers` vector.
    let doubled: Vec<i32> = numbers.iter().map(|item| item * 2).collect();
    dbg!(doubled);

    let taken: Vec<&i32> = numbers
        .iter()
        .take_while(|number| {
            return **number <= 2;
        })
        .collect();
    dbg!(taken);

    // `into_iter` method produces iterator which takes ownership of `numbers` vector.
    let filtered: Vec<i32> = numbers.into_iter().filter(|number| *number >= 2).collect();
    // `numbers` cannot be used here.
    dbg!(filtered);
}
