//! Do not communicate by sharing memory; instead, share memory by communicating;
//!
//! ## mpsc
//!
use std::{sync::mpsc, thread};
use std::fmt::Debug;
use std::time::Duration;

pub fn passing_message() {
    // create channel: transmitter and receiver object
    let (tx, rx) = mpsc::channel();
    // spawn thread to send message
    thread::spawn(move || {
        // message
        let val = String::from("hi");
        // send message: ownership transferred
        tx.send(val).unwrap();
    });
    // create receiver
    let received = rx.recv().unwrap();
    println!("Message received: {}", received);
}

pub fn passing_message_with_sleep_durations() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("spawned"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Message received: \"{}\" at {:?}", received, std::time::SystemTime::now());
    }
}