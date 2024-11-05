fn main() {
    let mut s = String::from("hello");

    println!("size before mutate: {}", get_length(&s));

    mutate(&mut s);

    println!("size after mutate: {}", get_length(&s));
}

fn get_length(string: &String) -> usize {
    string.len()
}

fn mutate(string: &mut String) {
    string.push_str(", World!");
}
