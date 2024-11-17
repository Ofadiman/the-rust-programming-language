use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Clone)]
struct Greater {}

impl Greater {
    pub fn hello(&self, delay: u64) {
        println!(
            "[spawned:{:?}] Hello after {} milliseconds!",
            thread::current().id(),
            delay
        )
    }
}

impl Drop for Greater {
    // Drop function is only called once.
    fn drop(&mut self) {
        println!("[main:{:?}] Greater dropped!", thread::current().id())
    }
}

// [spawned:ThreadId(2)] Hello after 0 milliseconds!
// [main:ThreadId(1)] Print after 500 milliseconds
// [spawned:ThreadId(3)] Hello after 314 milliseconds!
// [main:ThreadId(1)] Print after 1000 milliseconds
// [spawned:ThreadId(4)] Hello after 628 milliseconds!
// [spawned:ThreadId(5)] Hello after 942 milliseconds!
// [main:ThreadId(1)] Print after 1500 milliseconds
// [spawned:ThreadId(6)] Hello after 1256 milliseconds!
// [main:ThreadId(1)] Print after 2000 milliseconds
// [spawned:ThreadId(7)] Hello after 1570 milliseconds!
// [spawned:ThreadId(8)] Hello after 1884 milliseconds!
// [main:ThreadId(1)] Print after 2500 milliseconds
// [spawned:ThreadId(9)] Hello after 2198 milliseconds!
// [spawned:ThreadId(10)] Hello after 2512 milliseconds!
// [spawned:ThreadId(11)] Hello after 2826 milliseconds!
// [main:ThreadId(1)] Greater dropped!
fn main() {
    let mut handles: [Option<JoinHandle<()>>; 10] = Default::default();
    let greater = Arc::new(Greater {});

    for i in 0..handles.len() {
        let greater_copy = greater.clone();

        let handle = thread::spawn(move || {
            let delay = 314 * i as u64;
            thread::sleep(Duration::from_millis(delay as u64));
            greater_copy.clone().hello(delay);
        });

        handles[i as usize] = Some(handle);
    }

    for i in 1..=5 {
        println!(
            "[main:{:?}] Print after {} milliseconds",
            thread::current().id(),
            i * 500
        );
        thread::sleep(Duration::from_millis(500));
    }

    // The main thread can finish faster than side threads, so the code in the side threads might not have enough time to execute before closing the program.
    // In order to wait for the completion of side threads, they must be joined to the main thread.
    // The place where the join occurs matters and can block the main thread.
    for handle in handles {
        handle.unwrap().join().unwrap();
    }
}
