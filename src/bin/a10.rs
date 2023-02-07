// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn display_result(result:bool) {
    let message = match result {
        true => String::from("Its small"),
        false => String::from("Its big")
    };

    println!("{message}");
}

fn main() {
    let some_number = 100;
    let result = some_number <= 100;

    display_result(result);
}

