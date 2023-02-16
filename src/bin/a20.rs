// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io::stdin;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state_name: &str) -> Option<Self> {
        let name = state_name.trim().to_lowercase();
        match name.as_str() {
            "off" => Some(Self::Off),
            "sleep" => Some(Self::Sleep),
            "reboot" => Some(Self::Reboot),
            "shutdown" => Some(Self::Shutdown),
            "hibernate" => Some(Self::Hibernate),
            _ => None,
        }
    }

    fn print(&self) {
        match self {
            Self::Off => println!("Turn off computer."),
            Self::Sleep => println!("Sleep mode on."),
            Self::Reboot => println!("Starting reboot command."),
            Self::Shutdown => println!("Shuting down the OS."),
            Self::Hibernate => println!("Entering hibernation mode."),
        }
    }
}

fn main() {
    println!("Enter a command for the computer: ");
    let mut buffer = String::new();
    let user_input_status = stdin().read_line(&mut buffer);
    
    if !user_input_status.is_ok() {
        println!("Error trying to read command.");
        return;
    }

    match PowerState::new(&buffer) {
        Some(s) => s.print(),
        None => println!("Command not found."),
    }
}

