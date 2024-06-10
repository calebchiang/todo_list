use std::io;
use chrono::NaiveDate;
enum Priority {
    Low,
    Medium,
    High,
}

struct Task {
    id: u16,
    name: String,
    description: String,
    priority: Priority,
    completed: bool,
    due_date: chrono::NaiveDate,
}

fn get_user_input() -> u8 {
    loop {
        println!("Welcome to your To-do List");
        println!("Choose an option:");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Edit Task");
        println!("5. Remove Task");
        println!("6. Exit");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        match user_input.trim().parse::<u8>() {
            Ok(num) if num >= 1 && num <= 6 => return num,
            _ => {
                println!("Invalid input. Please enter a number between 1 and 6:")
            }
        }
    }
}

fn process_user_input(user_input: u8) {
    match user_input {
        1 => add_task(),
        2 => display_tasks(),
        3 => mark_completed(),
        4 => edit_task(),
        5 => remove_task(),
        6 => return,
        _ => println!("Invalid option. Please try again."),
    }
}

fn add_task() {
    println!("Task Name:");
    let mut task_name = String::new();
    io::stdin().read_line(& mut task_name).expect("Failed to read line");
    task_name = task_name.trim().to_string();

    println!("Description: ");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");
    description = description.trim().to_string();

    println!("Priority (high/medium/low):");
    let mut priority_input = String::new();
    io::stdin().read_line(&mut priority_input).expect("Failed to read line");
    let priority = match priority_input.trim().to_lowercase().as_ref() {
        "high" => Priority::High,
        "medium" => Priority::Medium,
        "low" => Priority::Low,
        _ => {
            println!("Invalid priority. Defaulting to low");
            Priority::Low
        },
    };

    println!("Due Date: ");
    let mut date_input = String::new();
    io::stdin().read_line(&mut date_input).expect("Failed to read line");
    let due_date = NaiveDate::parse_from_str(date_input.trim(), "%Y-%m-%d")
        .expect("Invalid date format. Ensure it's YYYY-MM-DD");


    // Display entered information (for debugging purposes or confirmation)
    println!("Entered Task Name: {}", task_name);
    println!("Entered Description: {}", description);
    println!("Selected Priority: {:?}", priority_input);
    println!("Set Due Date: {}", due_date);

}


fn main() {
    let user_input = get_user_input();
    println!("{}", user_input);
}
