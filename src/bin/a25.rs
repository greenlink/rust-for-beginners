// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
// * The function must utilize impl trait as a function parameter

struct Square{
    side : i32,
}

struct Triangle{
    side_a : i32,
    side_b : i32,
    side_c : i32,
}

trait Shape {
    fn get_perimeter(&self) -> i32;
}

impl Shape for Triangle {
    fn get_perimeter(&self) -> i32 {
        self.side_a+self.side_b+self.side_c
    }
}

impl Shape for Square {
    fn get_perimeter(&self) -> i32 {
        self.side*4
    }
}

fn show_perimeter(thing: impl Shape){
    println!("Perimeter: {:?}", thing.get_perimeter())
}

fn main() {
    show_perimeter(Triangle{side_a: 2, side_b: 3, side_c: 4});
    show_perimeter(Square{side: 2});
}
