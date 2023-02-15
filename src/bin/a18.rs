// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    name: String,
    age: u8
}

impl Customer {
    fn is_customer_adult(&self) -> Result<String, String> {
        if self.age >= 21 {
            return Ok("is legally adult.".to_owned());
        }

        return Err("is underaged.".to_owned());
    }

    fn can_purchase(&self) {
        match self.is_customer_adult() {
            Ok(msg) => println!("Customer named {0} {1}", self.name, msg),
            Err(error) => println!("Customer named {0} {1}", self.name, error)
        }
    }
}

fn main() {
    let harry = Customer {name: "Harry".to_owned(), age: 12};
    let megan = Customer {name: "Megan".to_owned(), age: 24};
    
    harry.can_purchase();
    megan.can_purchase();
}

