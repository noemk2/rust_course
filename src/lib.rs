#![allow(dead_code)]

// use {variables}

// mod main;
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

pub fn inspect(arg: &String) {
    // if arg is plural
    if arg.ends_with("s") {
        println!("{} is plural", arg);
    } else {
        println!("{} is not plural", arg);
    }
}

pub fn change(arg: &mut String) {
    if !arg.ends_with("s") {
        arg.push_str("s");
        return println!("add s ...  {}", arg);
    }
    return println!("ya tiene s la palabra '{}'", arg);
}

pub fn eat(consumes: &String) -> bool {
    if consumes.starts_with("b") && consumes.contains("a") {
        println!("Might be a bananas");
        return true;
    }
    println!("Not a bananas");
    return false;
}

// dereferences a reference
pub fn add(arg_a: &i32, arg_b: &i32) -> i32 {
    let _a = *arg_a;
    let _b = *arg_b;

    return _a + _b;
}

//   print_difference(coords);
//   print_array(coords_array);
// arg series 13
// ding(series[6]);

// let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
//         println!("Please supply an argument to this program.");
//         std::process::exit(-1);
//     });

#[derive(Debug)]
pub struct Grapes {
    pub amount_left: i32,
}

pub trait Bite {
    fn bite(self: &mut Self);
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

pub fn bunny_nibbles<T: Bite>(grapes: &mut T) {
    grapes.bite();
}

// let mut grapes = Grapes { amount_left: 100 };
// grapes.bite();
// println!("Eat a grape: {:?}", grapes);
// bunny_nibbles( &mut grapes );
// println!("Bunny nibbles for awhile: {:?}", grapes);

// collections_enums
pub enum Shot<'a> {
    Bullseye,
    Hit(&'a f64),
    Miss,
}

impl Shot<'_> {
    pub fn points(&self) -> i32 {
        match self {
            Shot::Bullseye => 5,
            Shot::Hit(x) => if_x(x),
            Shot::Miss => 1,
        }
    }
}

fn if_x(x: &f64) -> i32 {
    if *x < 3.0 {
        return 2;
    }
    if *x >= 3.0 {
        return 1;
    }
    return x.to_string().parse::<i32>().unwrap();
}

#[derive(Debug)]
pub struct Coord {
   pub x: f64,
   pub y: f64,
}

impl Coord {
    pub fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    

    pub fn print_description(&self) {
        println!(
            "Coordinates is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y
        );
    }
}

pub fn get_arrow_coords(arrow_num: i32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..arrow_num{
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}

pub fn sum_shots(shots: &Vec<i32>){
    let mut sum = 0;
    for shot in shots {
        sum += shot;
    }
    println!("Final point total is {}", sum);
}