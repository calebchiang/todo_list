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

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u16,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self) {
        println!("Task Name:");
        let mut task_name = String::new();
        io::stdin().read_line(&mut task_name).expect("Failed to read line");
        let task_name = task_name.trim().to_string();

        println!("Description:");
        let mut description = String::new();
        io::stdin().read_line(&mut description).expect("Failed to read line");
        let description = description.trim().to_string();

        // Loop for priority input
        let mut priority_input = String::new();
        let mut priority;
        loop {
            println!("Priority (high/medium/low):");
            priority_input.clear();  // Clear previous input
            io::stdin().read_line(&mut priority_input).expect("Failed to read line");
            priority = match priority_input.trim().to_lowercase().as_ref() {
                "high" => Priority::High,
                "medium" => Priority::Medium,
                "low" => Priority::Low,
                _ => {
                    println!("Invalid priority. Please enter 'high', 'medium', or 'low'.");
                    continue;
                },
            };
            break;  // Exit loop if valid priority is entered
        }

        // Loop for due date input
        println!("Due Date (YYYY-MM-DD):");
        let mut due_date;
        loop {
            let mut date_input = String::new();
            io::stdin().read_line(&mut date_input).expect("Failed to read line");
            match NaiveDate::parse_from_str(date_input.trim(), "%Y-%m-%d") {
                Ok(date) => {
                    due_date = date;
                    break;  // Exit loop if date is valid
                },
                Err(_) => {
                    println!("Invalid date format. Ensure it's YYYY-MM-DD. Please try again.");
                }
            }
        }

        let task = Task {
            id: self.next_id,
            name: task_name,
            description: description,
            priority: priority,
            completed: false,
            due_date: due_date,
        };

        self.tasks.push(task);
        self.next_id += 1;

        println!("Task added successfully. Task ID: {}", self.next_id - 1);
    }
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

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        let user_input = get_user_input();
        match user_input {
            1 => task_manager.add_task(),
            2 => println!("View Tasks functionality to be implemented."),
            3 => println!("Mark Task as Completed functionality to be implemented."),
            4 => println!("Edit Task functionality to be implemented."),
            5 => println!("Remove Task functionality to be implemented."),
            6 => {
                println!("Exiting the program.");
                break;
            },
            _ => println!("Invalid option. Please try again."),
        }
    }
}
