// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let stock_itens = HashMap::from([("Chairs", 5), ("Beds", 3), ("Tables", 2), ("Couches", 0)]);
    let mut total_itens = 0;

    for (name, quantity) in stock_itens.iter() {
        match quantity {
            0 => println!("Item: {0}. Quantity: Out of stock.", name),
            _ => println!("Item: {0}. Quantity: {1}.", name, quantity),
        }

        total_itens += quantity;
    }

    println!("Total numbers of itens: {:?}.", total_itens);
}

