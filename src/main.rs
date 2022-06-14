// TITLE: collections_enums
#![allow(unused_variables, dead_code, unused_imports)]

// use log::{info, trace, warn};

// use variables::{Grapes, bunny_nibbles, Bite};
use variables::Shot;
use variables::{get_arrow_coords, Coord, sum_shots};

fn main() {
    let arrwow_crowds: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<i32> = Vec::new();
    // let mut shots: Vec<Shot> = Vec::new();
    // let numero : i32 = Shot::Bullseye.points();
    // println!("{:?}", numero);

    // iterarte over the vector
    for coord in &arrwow_crowds {
        // let _ = coord.distance_from_center();
        let one_coord: &f64 =  &coord.distance_from_center();

        if one_coord < &1.0 {
            shots.push(Shot::Bullseye.points());
        }
        if one_coord >= &1.0 && one_coord <= &5.0 {
           shots.push(Shot::Hit(one_coord).points());
        }
        if one_coord > &5.0 {
           shots.push(Shot::Miss.points());
        }

    }

    println!("{:?}", &shots);
    // let total_points = sum_shots(shots);
    println!("{:?}", sum_shots(&shots));

    // let mut shots: Vec<Shot> = Vec::new();

    // println!( "Arrow coords: {:?}", arrwow_crowds );
    // info!("this is {:?}", arrwow_crowds);
}
