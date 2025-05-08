#[allow(dead_code,unused_variables)]
fn example_0(){
    let x : i32;

    {
        let y : i32 = 4;
        //x = &y;               <-- Error: `y` does not live long enough
    }
    //println!("x = {}", x);    <--- Error: `x` does not live long enough
}

#[allow(dead_code,unused_variables)]
fn example_1(){
    let height_age : i32;

    let alice_age : i32 = 20;
    let bob_age : i32 = 21;

    height_age = compare_ages(&alice_age, &bob_age);

    println!("Alice's age = {}", height_age);

    fn compare_ages(compare_1 : &i32, compare_2 : &i32) -> i32 {
        if(compare_1 > compare_2) {
            *compare_1
        } else {
            *compare_2
        }
    }
}


#[allow(dead_code,unused_variables)]
fn example_2(){
    let height_age : i32;

    let alice_age : i32 = 20;
    let bob_age : i32 = 21;

    height_age = compare_ages(&alice_age, &bob_age);

    println!("Alice's age = {}", height_age);

    fn compare_ages(compare_1 : &i32, compare_2 : &i32) -> i32 {
        if(compare_1 > compare_2) {
            *compare_1
        } else {
            *compare_2
        }
    }
}
