


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_literals() {
        let number : i32 = 20;

        match number {
            1 => println!("One"),
            2 | 3 | 4 |5 => println!("More than one"),
            _ => println!("Not a number between 1 and 5"),
        }

    }

    #[test]
    fn test_match_options() {
        
        let some_num : Option<i32> = Some(20);
        let some_none : Option<i32> = None;

        match some_num {
            Some(x) => println!("Some number: {}", x),
            None => println!("No number"),
        }

        let result = match some_none {
            Some(x) => x,
            None => panic!("There was a problem"),
        };

        println!("Result: {}", result);


    }

    #[test]
    fn test_match_option2() {
        
        let some_num : Option<i32> = Some(20);
        let my_num = if let Some(i) = some_num {
            i
        } else {
            panic!("No number");
        };

        println!("My number: {}", my_num);


    }

    #[test]
    fn test_match_result() {
        
        let res : Result<i32, String> = Ok(20);
        let err : Result<i32, String> = Err("Error".to_string());

        match res {
            Ok(x) => println!("Result: {}", x),
            Err(e) => println!("Error: {}", e),
        };

        match err {
            Ok(x) => println!("Result: {}", x),
            Err(e) => println!("Error: {}", e),
        };

    }
}