// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn check_value(value:i32) {
    if value > 5 {
        println!("{value} > 5");
    }
    else if value < 5 {
        println!("{value} < 5");
    }
    else {
        println!("{value} == 5")
    }
}

fn main() {
    let a = 6;
    let b = 4;
    let c = 5;

    check_value(a);
    check_value(b);
    check_value(c);
}

