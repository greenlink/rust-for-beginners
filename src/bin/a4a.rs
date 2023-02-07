// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn display_result(result:bool) {
    match result {
       true => println!("It's true"),
       false => println!("It's false"),
    }
}

fn main() {
    let truth = true;
    let fake = false;

    display_result(truth);
    display_result(fake);
}

