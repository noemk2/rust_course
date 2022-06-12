#![allow(unused_variables)]
// use std::env as enva;

mod lib;

// TITLE: comand-line arguments

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        //if arg == "sum"
        if arg == "sum" {
            return lib::sum();
        }

        if arg == "double" {
            return lib::double();
        }

        return lib::count(arg);
    }
}
