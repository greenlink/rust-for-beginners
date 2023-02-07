// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Red,
    Blue,
    Green,
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn new(width: f64, height: f64, depth: f64) -> Self {
        Dimensions {
            width: width,
            height: height,
            depth: depth,
        }
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    fn new(dimensions: Dimensions, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            dimensions: dimensions,
            weight: weight,
            color: color,
        }
    }

    fn display_box(&self) {
        println!(
            "Dimensions: {0}inX{1}inX{2}in",
            self.dimensions.width, self.dimensions.height, self.dimensions.depth
        );
        println!("Weight: {}lbs", self.weight);
        match self.color {
            BoxColor::Red => println!("Box color: Red."),
            BoxColor::Blue => println!("Box color: Blue."),
            BoxColor::Green => println!("Box color: Green."),

        }
    }
}

fn main() {
    let box_new = ShippingBox::new(Dimensions::new(4.5, 3.0, 11.0), 64.0, BoxColor::Green);
    
    box_new.display_box();
}

