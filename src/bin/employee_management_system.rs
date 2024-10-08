use std::collections::HashMap;
use std::io;

fn main() {
    let mut depart_wise_employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let option = display_main_screen();
        match option {
            0 => break,
            1 => store_employee_details(&mut depart_wise_employees),
            2 => print_employees_from_a_department(&depart_wise_employees),
            3 => print_all_employees_department_wise(&depart_wise_employees),
            _ => continue,
        }
    }
}

fn display_main_screen() -> u8 {
    println!("*************** MAIN SCREEN **************");
    println!("0. Exit");
    println!("1. Store Employee");
    println!("2. Print All Employees In a Department Sorted By Employee Name");
    println!("3. Print All Employees Department Code wise Sorted By Employee Name");
    println!();
    println!("Enter Option Number");
    let mut option_string = String::new();

    io::stdin().read_line(&mut option_string).unwrap();
    let option: u8 = option_string.trim_end().parse().unwrap();

    option
}

fn store_employee_details(depart_wise_employees: &mut HashMap<String, Vec<String>>) {
    // Prompt for employee name
    println!("Enter Employee Name:");
    let mut employee_name = String::new();
    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line");
    let employee_name = employee_name.trim_end().to_string();

    // Prompt for employee department
    println!("Enter Employee Department:");
    let mut employee_department = String::new();
    io::stdin()
        .read_line(&mut employee_department)
        .expect("Failed to read line");
    let employee_department = employee_department.trim_end().to_string();

    // Use the entry API to insert or modify the department's employee list
    depart_wise_employees
        .entry(employee_department)
        .or_default()
        .push(employee_name);
}

fn print_sorted_employees_for_a_department(
    department: &String,
    depart_wise_employees: &HashMap<String, Vec<String>>,
) {
    let mut employees = depart_wise_employees
        .get(department)
        .expect("No employees found in this department")
        .clone();
    employees.sort();

    println!("*******************");
    println!("Employees In {department}");
    println!("*******************");
    for employee in employees {
        println!("{employee}");
    }
    println!("*******************");
}

fn print_employees_from_a_department(depart_wise_employees: &HashMap<String, Vec<String>>) {
    // Prompt for employee department
    println!("Enter Employee Department from which employees needs to be displayed:");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");
    let department = department.trim_end().to_string();

    print_sorted_employees_for_a_department(&department, depart_wise_employees);
}

fn print_all_employees_department_wise(depart_wise_employees: &HashMap<String, Vec<String>>) {
    for department in depart_wise_employees.keys() {
        print_sorted_employees_for_a_department(department, depart_wise_employees);
    }
}
