// Using move Closures with Threads
// The `move` closure is often used alongside thread::spawn because it allows you to 
// use data from one thread in another thread.
// 
// Example 1: Attempting to use a vector created by the main thread in another thread
//
// Example 2: A thread with a closure that attempts to capture a reference to v from 
// a main thread that drops v
//
// Example 3: Using the move keyword to force a closure to take ownership of the 
// values it uses
//
// Example 4: What would happen to the code where the main thread called drop if we 
// use a move closure? Would move fix that case? Unfortunately, no.

use std::thread;

// Example 1
//
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// Example 2
//
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}

// Example 3
//
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}