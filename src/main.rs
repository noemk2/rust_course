// TITLE: procesador de imagenes
#![allow(unused_variables, dead_code, unused_imports)]

use variables::{blur, fractal, print_usage_and_exit};

fn main() {
    // use clap future

    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        print_usage_and_exit();
    }

    let subcommand = args.remove(0);

    // vefificar
    println!("{}", subcommand);

    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(infile, outfile);
        }

        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }
        _ => {
            print_usage_and_exit();
        }
    } // match
} //main
