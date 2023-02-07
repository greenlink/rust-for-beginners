// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn display_greeting(coming: bool) {
    if coming == true {
        println!("Hello!");
    }
    else {
        println!("Goodbye!");
    }
}

fn main() {
    let a = true;
    let b = false;
    display_greeting(a);
    display_greeting(b);
}

