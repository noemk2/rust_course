// TITLE: collections_enums
#![allow(unused_variables, dead_code, unused_imports)]

// use log::{info, trace, warn};

// use variables::{Grapes, bunny_nibbles, Bite};
// use variables::Shot;
// use variables::{get_arrow_coords, sum_shots, Coord, iterate_over_vector};

// use std::thread;

// closures_threads
// use crossbeam::channel;
// use variables::{expensive_sum, pause_ms};

// use std::sync::mpsc ;
use std::sync::mpsc::channel;
use std::thread;
use variables::pause_ms;

fn main() {
    let (tx, rx) = channel();

    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(5000);
        tx.send("Hello from thread A".to_string()).unwrap();
    });

    let handle_b = thread::spawn(move || {
        tx2.send("Hello from thread B".to_string()).unwrap();
    });

    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    handle_a.join().unwrap();
    handle_b.join().unwrap();
}

/*
std::sync::mpsc module
este modulo tambien sirve para enviar y recibir mensajes entre procesos
*/
