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
    let height_age : &i32;

    let alice_age : i32 = 20;
    let bob_age : i32 = 21;

    height_age = compare_ages(&alice_age, &bob_age);

    //Do not
    {
        
        //let bob_age : i32 = 21;

        //height_age = compare_ages(&alice_age, &bob_age); // <-- Error: `bob_age` does not live long enough

    }

    println!("Alice's age = {}", height_age);

    fn compare_ages<'a>(compare_1 : &'a i32, compare_2 : &'a i32) -> &'a i32 {
        if(compare_1 > compare_2) {
            compare_1
        } else {
            compare_2
        }
    }
}




#[allow(dead_code,unused_variables)]
fn example_3(){
    let height_age : &i32;

    let alice_age : i32 = 20;
    let bob_age : i32 = 21;

    height_age = compare_ages::<i32>(&alice_age, &bob_age);

    println!("Alice's age = {}", height_age);

    fn compare_ages<'a, T:PartialOrd>(compare_1 : &'a T, compare_2 : &'a T) -> &'a T {
        if(compare_1 > compare_2) {
            compare_1
        } else {
            compare_2
        }
    }
}


#[allow(dead_code,unused_variables)]
struct Person<'p> {
    name: &'p str,
    age: &'p i32,
}

#[allow(dead_code,unused_variables)]
fn example_4_with_struct(){
    let height_age : &i32;
    let new_value : i32;

    let peter = Person {
        name: "Peter",
        age: &25,
    };

    {
        let bob  = Person {
            name: "Bob",
            age: &21,
        };

        height_age = compare_ages(peter.age, bob.age); // <-- Error: `bob` does not live long enough
        new_value = *height_age; // <-- Error: `bob` does not live long enough
    }

    println!("Compare's age = {}", new_value);

    fn compare_ages<'a, T:PartialOrd>(compare_1 : &'a T, compare_2 : &'a T) -> &'a T {
        if(compare_1 > compare_2) {
            compare_1
        } else {
            compare_2
        }
    }
}

