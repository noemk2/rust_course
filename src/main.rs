// TITLE: sctructs_traits   
#![allow(unused_variables)]
use variables::{Grapes, bunny_nibbles};

fn main() {
    let mut grapes = Grapes { amount_left: 100 };
    // grapes.bite();
    // println!("Eat a grape: {:?}", grapes);
    bunny_nibbles( &mut grapes );
    println!("Bunny nibbles for awhile: {:?}", grapes);
}

