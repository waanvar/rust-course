mod func;

use crate::func::add_five;

fn main() {
    println!("Add five from 5 is {}", add_five(5));

    let x : u8 = 5;

    println!("x is {}", x);

    let arr : Vec<u8> = vec![1, 2, 3, 4, 5];
    println!("arr is {:?}", arr);
    let mut arr2 : Vec<u8> = vec![1, 2, 3, 4, 5];
    arr2.push(6);
    println!("arr2 is {:?}", arr2);
    let mut arr3 : Vec<u8> = vec![1, 2, 3, 4, 5];
    arr3.push(6);
    arr3.push(7);
    println!("arr3 is {:?}", arr3);
    let mut arr4 : Vec<u8> = vec![1, 2, 3, 4, 5];
    arr4.push(6);
    arr4.push(7);
    arr4.push(8);
    println!("arr4 is {:?}", arr4);     

    let _arr  = &arr[0..3];
    println!("_arr is {:?}", _arr);

    let _arr2 = &arr2[0..3];
    println!("_arr2 is {:?}", _arr2);

    let mut x : u8 = 5;
    let y : &mut u8 = &mut x;
    *y += 1;
    println!("x is {}", x);
}
