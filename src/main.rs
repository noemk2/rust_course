#![allow(unused_variables)]


mod lib;

// TITLE: ownership_references

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    // println!("{}", arg);
    // lib::inspect(&arg);
    // lib::change(&mut arg);
    lib::eat(&arg);
}
