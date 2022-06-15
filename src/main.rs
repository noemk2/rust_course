// TITLE: procesador de imagenes
#![allow(unused_variables, dead_code, unused_imports)]

use variables::{blur, fractal, generate_options, print_usage_and_exit};

//  cargo run --release blur image.png blurred.png blur.png
//  cargo run --release blur image.png 

fn main() {
    // use clap future
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    // assert_eq!(args.is_empty(), vec![""].is_empty());

    // cargo run --release blur image.png blurred.png
    // println!("this is args: {:?}", args);

    // assert!(args.is_empty()); // wtf
    if args.is_empty() {
        print_usage_and_exit();
    }

    // fix : using two variables
    // let processing= args.clone().remove(0);
    let processing= args.remove(0);

    // vefificar
    // println!("this is process: {}", processing); // arg.1
    generate_options(&processing, args);

} //main
