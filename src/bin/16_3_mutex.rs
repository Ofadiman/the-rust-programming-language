use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter = counter.clone();

        let handle = thread::spawn(move || {
            let mut result = counter.lock().expect("mutex poisoning must not happen");

            *result += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
