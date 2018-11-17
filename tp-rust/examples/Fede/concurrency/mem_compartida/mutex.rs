use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter_mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let inner_counter_mutex = counter_mutex.clone();
        let handle = thread::spawn(move || {
            let mut counter = inner_counter_mutex.lock().unwrap();
            println!("Thread Nº {} has the lock", i);
            *counter += 1;
            println!("Thread Nº {} still has the lock", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter_mutex.lock().unwrap());
}