// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: f64, b: f64) -> Option<f64> {
    return if b != 0f64 {
        Some(a / b)
    } else {
        Some(0f64)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first.trim(), second.trim())
}

fn main() {}

#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn check_clamp(){
        let expected = 7;
        let result = clamp(7, 6,8);

        assert_eq!(result, expected, "Should return 'n'.");
    }

    #[test]
    fn check_clamp_is_not_lower(){
        let expected = 6;
        let result = clamp(5, 6,8);

        assert_eq!(result, expected, "Should return lower.");
    }

    #[test]
    fn check_clamp_is_not_upper(){
        let expected = 8;
        let result = clamp(9, 6,8);

        assert_eq!(result, expected, "Should return upper.");
    }

    #[test]
    fn check_div(){
        let expected = 4f64;
        let result = div(8f64, 2f64).unwrap();

        assert_eq!(result, expected, "Should return correct result.");
    }

    #[test]
    fn check_div_by_zero(){
        let expected = 0f64;
        let result = div(8f64, 0f64).unwrap();

        assert_eq!(result, expected, "Should return 0 since no number can be divided by 0.");
    }

    #[test]
    fn check_div_with_decimals(){
        let expected = 2.5;
        let result = div(5f64, 2f64).unwrap();

        assert_eq!(result, expected, "Should return 2 since is a integer.");
    }

    #[test]
    fn check_concat(){
        let expected = String::from("LoveGame");
        let result = concat("Love", "Game");

        assert_eq!(result, expected, "Should return correct result.");
    }

    #[test]
    fn check_concat_without_spacing(){
        let expected = String::from("LoveGame");
        let result = concat("Love      ", "Game ");

        assert_eq!(result, expected, "Should return correct result.");
    }
}
