#![allow(dead_code)]

use std::string::ToString;

pub fn print_difference(x: f32, y: f32) {
    // print!("{}", coords.0 - coords.1);
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

pub fn print_array(coords_array: [f32; 2]) {
    print!("{}", coords_array[0] - coords_array[1]);
}

pub fn ding(arg: i32) {
    if arg == 13 {
        return println!("Ding, you found 13!");
    }
    return println!(" its not 13 is :{}", arg);
}

pub fn on_off(data_true: bool) {
    if data_true {
        println!("Lights are on!");
    } else {
        println!("off");
    }
}

pub fn print_distance(coords: (f32, f32)) {
    // println!("Distance is {}",  (x - y).abs());
    // println!("Distance is {}", (coords.0 - coords.1).abs());
    println!(
        "Distance to the origin is {}",
        (coords.0.powf(2.0) + coords.1.powf(2.0)).sqrt()
    );
}

// pub fn sum(a: i32, b: i32) -> i32 {
//     a + b
// }

pub fn sum() {
    println!("sum function")
}

pub fn double() {
    println!("double function")
}

pub fn count<T: ToString>(x: T) {
    let s: String = x.to_string();
    println!("count {} function ", s);
}

pub fn control_flow_string() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        //if arg == "sum"
        if arg == "sum" {
            return sum();
        }

        if arg == "double" {
            return double();
        }
        return count(arg);
    }
}

//   print_difference(coords);
//   print_array(coords_array);
// arg series 13
// ding(series[6]);
