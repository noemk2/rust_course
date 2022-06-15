// TITLE: procesador de imagenes 
#![allow(unused_variables, dead_code, unused_imports)]


fn main() {
// use clap future 

let args: Vec<String> = std::env::args().skip(1).collect();

if args.is_empty() {
        print_usage_and_exit();
    }

let subcommand = args.remove(0);

println!("{}", subcommand);


match subcommand.as_str() {
    "convert" => {
        let input_file = args.remove(0);
        }
    }// match





}//main