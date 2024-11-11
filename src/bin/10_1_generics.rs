use std::borrow::Borrow;

fn main() {
    let i32s: Vec<i32> = vec![1, 5, 3];
    let largest_i32 = largest(i32s.borrow());
    println!("largest_i32: {largest_i32}");

    let f64s: Vec<f64> = vec![1.0, 5.0, 3.0];
    let largest_f64 = largest(f64s.borrow());
    println!("largest_f64: {largest_f64}");

    let point = Point { x: 1, y: 2.5 };
    println!("point: {point:?}");
    let coordinates = point.coordinates();
    println!("coordinates: {coordinates:?}");

    let enumerable = Maybe::Yes("Hello, World!".to_string());
    if let Maybe::Yes(value) = enumerable {
        println!("enumerable contains {value}");
    }
}

fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: Copy,
    U: Copy,
{
    fn coordinates(&self) -> (T, U) {
        (self.x.clone(), self.y.clone())
    }
}

#[allow(dead_code)]
enum Maybe<T> {
    Yes(T),
    No,
}
