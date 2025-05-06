#[derive(Debug)]
enum CarColour {
    Blue,
}

#[derive(Debug)]
enum GiveResult<T,E>{
    Ok(T),
    Err(E),
}


#[derive(Debug)]
enum GiveOption<T>{
    None,
    Some(T),
}

fn create_car_colour_blue() -> CarColour {
    let my_car_colour = CarColour::Blue;
    my_car_colour
}

fn check_under_five(num_check : u8) -> GiveResult<u8,String> {
    if num_check < 5 {
        GiveResult::Ok(num_check)
    } else {
        GiveResult::Err("Number is not less than 5".to_string())
    }
}

fn remainder_zero(num_check : u8) -> GiveOption<u8> {
    if num_check % 2 == 0 {
        GiveOption::Some(num_check)
    } else {
        GiveOption::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum() {
        let car_colour = create_car_colour_blue();
        dbg!(car_colour);

        let under_five_result = check_under_five(3);
        dbg!(under_five_result);

        let under_five_result = check_under_five(6);
        dbg!(under_five_result);

        let remainder_zero_result = remainder_zero(2);
        dbg!(remainder_zero_result);
        
    }

}