// TITLE: procesador de imagenes
// #![allow(unused_variables, dead_code, unused_imports)]

use variables::{generate_options, print_usage_and_exit};

//  cargo run --release blur image.png blurred.png blur.png
//  cargo run --release blur image.png

fn main() {
    // use clap future
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    // assert!(args.is_empty()); // wtf
    if args.is_empty() {
        print_usage_and_exit();
    }

    // let processing= args.clone().remove(0);
    let processing = args.remove(0);

    generate_options(&processing, args);
} 