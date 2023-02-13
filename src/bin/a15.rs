// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Tickets::Backstage(180.0, "Dylan".to_owned()),
        Tickets::Vip(120.0, "Maura".to_owned()),
        Tickets::Standard(80.0),
    ];

    for ticket in tickets {
        match ticket {
            Tickets::Standard(price) => println!("Standard holder, price is U$ {};", price),
            Tickets::Vip(price, holder) => {
                println!("Vip holder is called {1}, price is U$ {0};", price, holder)
            }
            Tickets::Backstage(price, holder) => println!(
                "Backstage holder is called {1}, price is U$ {0};",
                price, holder
            ),
        }
    }
}

