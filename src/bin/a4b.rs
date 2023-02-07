// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn display_number_as_text(number:i32) {
    match number {
       1 => println!("One"),
       2 => println!("Two"),
       3 => println!("Three"),
       _ => println!("Other"),
    }
}

fn main() {
    let one = 1;
    let two = 2;
    let three = 3;
    let four = 4;

    display_number_as_text(one);
    display_number_as_text(two);
    display_number_as_text(three);
    display_number_as_text(four);
}

