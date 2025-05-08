mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;

fn main() {
    println!("Hello, world!");


    let y : i32 = 4;
    println!("y = {}", y);

    for i   in 0..y{
        println!("i = {}", i);
    }

    let my_float : [f32;3] = [0.0; 3];
    println!("my_float = {:?}", my_float);

    let my_new_float : [f32;3] = my_float.map(|y| y + 1.0);
    println!("my_new_float = {:?}", my_new_float);
}
