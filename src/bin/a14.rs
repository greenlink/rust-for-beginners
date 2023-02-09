// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

fn print_text(text: &str) {
    println!("{text}");
}

struct Person {
    name: String,
    age: u32,
    favorite_color: String
}

impl Person {
    fn print_person(&self) {
        print_text(&["Person's name: ", &self.name].concat());
        print_text(&["Person's favorite color: ", &self.favorite_color].concat());
    }
}

fn main() {
    let people = vec![
        Person {name: "Alice".to_string(), age: 9, favorite_color: "Lilac".to_string()},
        Person {name: "Robert".to_string(), age: 33, favorite_color: "Blue".to_string()},
        Person {name: "Priscila".to_string(), age: 10, favorite_color: "Amaryllis".to_string()},
    ];

    for person in &people {
        if person.age <= 10 {
            person.print_person();
        }
    }
}

