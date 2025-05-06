/// This module contains a simple function that adds five to a given number.
/// It also includes a test module to verify the functionality of the function.
/// example usage:
/// ```rust
/// mod func;
/// use crate::func::add_five;
/// fn main() {
///   println!("Add five from 5 is {}", add_five(5));
/// }
/// ```
/// The `add_five` function takes an integer as input and returns the integer plus five.

pub fn add_five(x: i32) -> i32 {
    x + 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_five() {
        assert_eq!(add_five(5), 10);
        assert_eq!(add_five(0), 5);
        assert_eq!(add_five(-5), 0);
    }
}