// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum DrinkFlavors {
    Lemonade,
    Strawberry,
    Grape
}

struct Drink {
    ounces:f32,
    flavor:DrinkFlavors
}

fn display_drink_info(drink:Drink) {
    match drink.flavor {
       DrinkFlavors::Lemonade => println!("Pour {} oz of Lemonade.", drink.ounces),
       DrinkFlavors::Strawberry => println!("Pour {} oz of Straberry juice.", drink.ounces),
       DrinkFlavors::Grape => println!("Pour {} oz of Grape juice.", drink.ounces),
    }
}

fn main() {
    let lemonade = Drink {
        ounces : 7.5,
        flavor : DrinkFlavors::Lemonade
    };
    let strawberry = Drink {
        ounces : 5.0,
        flavor : DrinkFlavors::Strawberry
    };
    let grape = Drink {
        ounces : 11.1,
        flavor : DrinkFlavors::Grape
    };

    display_drink_info(lemonade);
    display_drink_info(strawberry);
    display_drink_info(grape);
}

