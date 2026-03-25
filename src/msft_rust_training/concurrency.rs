use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicI64::new(0));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1_000_000 {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    println!("counter: {}", counter.load(Ordering::Relaxed));

    let shared = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..4)
        .map(|i| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                let mut data = shared.lock().unwrap();
                data.push(i);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    println!("{:?}", shared.lock().unwrap());
}
