// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::collections::HashMap;
use std::io::stdin;
use crate::Menu::{AddBill, Exit, InvalidOption, RemoveBill, ShowBills, EditBill};

fn main() {
    let mut bills = HashMap::new();
    let mut menu_option = InvalidOption;
    while !matches!(menu_option, Exit) {
        Menu::show_menu();
        let menu_string =  get_command();
        menu_option = Menu::get_menu_option(menu_string);
        println!("****************");

        match menu_option {
            AddBill => Menu::add_bill(&mut bills),
            ShowBills => Menu::show_bills(&mut bills),
            RemoveBill => Menu::remove_bill(&mut bills),
            EditBill => Menu::edit_bill(&mut bills),
            Exit => println!("Exiting application."),
            _ => {}
        }
    }
}

enum Menu{
    InvalidOption = 0,
    AddBill = 1,
    ShowBills = 2,
    RemoveBill = 3,
    EditBill = 4,
    Exit = 99,
}

impl Menu {
    fn show_menu(){
        println!("***** MENU *****");
        println!("1 - Add bill");
        println!("2 - View bills");
        println!("3 - Remove bill");
        println!("4 - Edit bill");
        println!("99 - Exit");
    }

    fn get_menu_option(menu_option_str: String) -> Menu {
        let menu_option_parsed = menu_option_str.parse::<isize>().unwrap_or_else(|error| {
            println!("Error while parsing the chosen option: {error}");
            0
        });
        match menu_option_parsed {
            1 => AddBill,
            2 => ShowBills,
            3 => RemoveBill,
            4 => EditBill,
            99 => Exit,
            _ => {
                println!("Invalid option.");
                InvalidOption
            }
        }
    }

    fn add_bill(bills: &mut HashMap<usize, Bill>){
        println!("Enter the name of the bill:");
        let name = get_command();
        println!("Enter the amount to be paid(use . for decimals):");
        let amount = get_command().parse::<f32>().unwrap_or_else(|error| {
            println!("Error while parsing the amount: {error}");
            0f32
        });

        if !name.is_empty() && amount != 0f32 {
            let new_bill_key = match bills.keys().max() {
                Some(result) => result + 1,
                None => 1
            };
            bills.insert(new_bill_key, Bill::new(name, amount));
            println!("Bill added successfully!");
        }
    }

    fn show_bills(bills: & HashMap<usize, Bill>){
        for item in bills.iter() {
            let bill = item.1;
            println!("{} - Name: {}, Amount: ${}", item.0, bill.name, bill.amount.to_string())
        }
    }

    fn remove_bill(bills: &mut HashMap<usize, Bill>) {
        Self::show_bills(&bills);
        println!("Enter the number of the bill:");
        let bill_number = get_command().parse::<usize>().unwrap_or_else(|error| {
            println!("Error while parsing the amount: {error}");
            0
        });

        if bills.contains_key(&bill_number) {
            bills.remove(&bill_number);
        } else {
            println!("Invalid bill number.");
        }
    }

    fn edit_bill(bills: &mut HashMap<usize, Bill>) {
        Self::show_bills(&bills);
        println!("Enter the number of the bill:");
        let bill_number = get_command().parse::<usize>().unwrap_or_else(|error| {
            println!("Error while parsing the amount: {error}");
            0
        });

        if bills.contains_key(&bill_number) {
            println!("Enter the new name of the bill:");
            let name = get_command();
            println!("Enter the new amount to be paid(use . for decimals):");
            let amount = get_command().parse::<f32>().unwrap_or_else(|error| {
                println!("Error while parsing the amount: {error}");
                0f32
            });

            bills.get_mut(&bill_number).unwrap().name = name;
            bills.get_mut(&bill_number).unwrap().amount = amount;
        } else {
            println!("Invalid bill number.");
        }
    }
}

struct Bill{
    name: String,
    amount: f32,
}

impl Bill {
    fn new(name: String, amount: f32) -> Self{
        Bill{
            name,
            amount
        }
    }
}

fn get_command() -> String{
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => println!("Error while trying to get input: {error}"),
    }

    input.trim().to_string()
}