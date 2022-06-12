#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    {
        let area = area_of(width, height);
    }

    println!("Area is {}", area_of(width, height));
    println!("Volume is {}", volume(width, height, depth));
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}


fn area_of(x: i32, y: i32) -> i32 {
    return x*y;
}
