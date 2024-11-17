use chrono::Utc;
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender, receiver) = mpsc::channel::<String>();

    for delay in [911, 1917, 6771] as [u64; 3] {
        let sender = sender.clone();

        thread::spawn(move || {
            let thread_id = thread::current().id();

            for j in 1..=u64::MAX {
                let now = Utc::now();

                sender
                    .send(format!(
                        "[{thread_id:?} at {now}] message #{j} from thread delayed by {delay} milliseconds",
                    ))
                    .expect("sending message must not fail");

                thread::sleep(Duration::from_millis(delay));
            }
        });
    }

    for message in receiver {
        println!("message: {message}");
    }
}
