fn main() {
    println!("2+2={}", add(2, 2))
}

fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code)]
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left > right {
        left
    } else {
        right
    }
}

#[allow(dead_code)]
fn might_panic(number: u32) {
    if number > 100 {
        panic!("number must not be greater than 100")
    }

    if number < 10 {
        panic!("number must not be less than 10")
    }
}

// The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.
#[cfg(test)]
mod tests {
    use super::*;

    mod add {
        use super::*;

        #[test]
        fn exploration() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }

    mod longest {
        use super::*;

        #[test]
        fn should_return_longest_string() {
            let result = longest("first", "second");
            assert_eq!(result, "second");
        }
    }

    mod panicking {
        use super::*;

        #[test]
        #[should_panic(expected = "number must not be greater than 100")]
        fn should_panic_when_number_is_greater_than_100() {
            might_panic(101)
        }

        #[test]
        #[should_panic(expected = "number must not be less than 10")]
        fn should_panic_when_number_is_less_than_10() {
            might_panic(9)
        }
    }
}
