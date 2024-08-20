// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoesColor(Color);

impl ShoesColor{
    fn new(color: Color) -> Self{
        Self(color)
    }
}

#[derive(Debug)]
struct ShirtColor(Color);

impl ShirtColor{
    fn new(color: Color) -> Self{
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);

impl PantsColor{
    fn new(color: Color) -> Self{
        Self(color)
    }
}

fn print_shoes_color(shoes_color: ShoesColor){
    println!("shoes color is {:?}", shoes_color.0);
}

fn print_shirt_color(shirt_color: ShirtColor){
    println!("shirt color is {:?}", shirt_color.0);
}

fn print_pants_color(pants_color: PantsColor){
    println!("pants color is {:?}", pants_color.0);
}
fn main() {
    print_shoes_color(ShoesColor::new(Color::Black));
    print_shoes_color(ShoesColor::new(Color::Blue));
    print_shoes_color(ShoesColor::new(Color::Brown));
    print_shoes_color(ShoesColor::new(Color::Custom(String::from("Orange"))));
    print_shirt_color(ShirtColor::new(Color::Gray));
    print_shirt_color(ShirtColor::new(Color::Green));
    print_shirt_color(ShirtColor::new(Color::Purple));
    print_pants_color(PantsColor::new(Color::Red));
    print_pants_color(PantsColor::new(Color::White));
    print_pants_color(PantsColor::new(Color::Yellow));
}
