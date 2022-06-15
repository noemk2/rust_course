// TITLE: procesador de imagenes
#![allow(unused_variables, dead_code, unused_imports)]

use variables::{blur, fractal, generate_options, print_usage_and_exit};

fn main() {
    // use clap future
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    // assert_eq!(args.is_empty(), vec![""].is_empty());


    // assert!(args.is_empty()); // wtf
    if args.is_empty() {
        print_usage_and_exit();
    }

    // fix : using two variables
    let subcommand_string = args.remove(0);

    // vefificar
    // println!("this is subcommand: {}", subcommand); // arg.1

    generate_options(&subcommand_string, args.clone());
} //main
