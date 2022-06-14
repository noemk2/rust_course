// TITLE: collections_enums
#![allow(unused_variables, dead_code, unused_imports)]

// use log::{info, trace, warn};

// use variables::{Grapes, bunny_nibbles, Bite};
use variables::Shot;
use variables::{get_arrow_coords, sum_shots, Coord, iterate_over_vector};


fn main() {
    let arrwow_crowds: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<i32> = Vec::new();

    iterate_over_vector( &mut shots, &arrwow_crowds);

    println!("{:?}", &shots);
    println!("{:?}", sum_shots(&shots));
}
