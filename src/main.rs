#![allow(unused_variables)]

mod lib;

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    let coords_array: [f32; 2] = [6.3, 15.0];
    let series = [1, 1, 2, 3, 5, 8, 13];
    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");

    lib::on_off(mess.2[1].0);
}
