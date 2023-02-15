// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeePosition {
    AssemblyTechnicians,
    KitchenStaff,
    LineSupervisor,
    MaintenanceCrew,
    Manager,
    MarketingDepartment,
}

struct Employee {
    employee_position: EmployeePosition,
    employed: bool,
}

impl Employee {
    fn check_access(&self) -> Result<(), String> {
        let no_access = "Employee can't access the building.".to_owned();

        if !self.employed {
            return Err(no_access);
        }

        match self.employee_position {
            EmployeePosition::MaintenanceCrew => return Ok(()),
            EmployeePosition::Manager => return Ok(()),
            EmployeePosition::MarketingDepartment => return Ok(()),
            _ => return Err(no_access),
        }
    }

    fn print_access(&self) -> Result<(), String> {
        self.check_access()?;
        println!("Employee can access the building.");
        Ok(())
    }
}

fn main() {
    let manager = Employee {
        employee_position: EmployeePosition::Manager,
        employed: true,
    };
    let manager_fired = Employee {
        employee_position: EmployeePosition::Manager,
        employed: false,
    };
    let staff = Employee {
        employee_position: EmployeePosition::KitchenStaff,
        employed: true,
    };
    let staff_fired = Employee {
        employee_position: EmployeePosition::KitchenStaff,
        employed: false,
    };

    let msg1 = manager.print_access();
    println!("{:?}", msg1);
    let msg2 = manager_fired.print_access();
    println!("{:?}", msg2);
    let msg3 =staff.print_access();
    println!("{:?}", msg3);
    let msg4 = staff_fired.print_access();
    println!("{:?}", msg4);
}

