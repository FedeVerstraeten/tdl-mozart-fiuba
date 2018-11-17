use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let sender2 = sender.clone();

    thread::spawn(move || {
        let message = String::from("message");
        sender.send(message).unwrap();
    });

    thread::spawn(move || {
        let message2 = String::from("message2");
        sender2.send(message2).unwrap();
    });

    // Recv bloqueante
    for _ in 0..2 {
        println!("Received: {}", receiver.recv().unwrap());
    }
}