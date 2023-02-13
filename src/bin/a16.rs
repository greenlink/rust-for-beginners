// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>
}

impl Student {
    fn print(&self) {
        print!("Student's name is {}, ", self.name);
        match self.locker {
            Some(number) => println!("locker {} assigned.", number),
            None => println!("have no Locker.")
        };
    }
}

fn main() {
    let mark = Student {name: String::from("Mark"), locker: Some(14)};
    let helly = Student {name: String::from("Helly"), locker: None};

    mark.print();
    helly.print();
}

