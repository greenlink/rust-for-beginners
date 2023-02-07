// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Blue,
    Green,
    Pink,
    Olive
}

fn display_color(color:Colors) {
    match color {
       Colors::Red => println!("Red"), 
       Colors::Blue => println!("Blue"), 
       Colors::Green => println!("Green"), 
       Colors::Pink => println!("Pink"), 
       Colors::Olive => println!("Olive") 
    }
}

fn main() {
    let red_color = Colors::Red;
    let blue_color = Colors::Blue;
    let green_color = Colors::Green;
    let pink_color = Colors::Pink;
    let olive_color = Colors::Olive;

    display_color(red_color);
    display_color(blue_color);
    display_color(green_color);
    display_color(pink_color);
    display_color(olive_color);
}

