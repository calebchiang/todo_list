use std::io;
use chrono::NaiveDate;
#[derive(Debug)]
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

    fn validate_id(&mut self) -> Option<&mut Task> {
        if self.tasks.is_empty() {
            println!("No tasks available.");
            return None;
        }

        println!("Enter the ID of the task:");
        let mut id_input = String::new();
        std::io::stdin().read_line(&mut id_input).expect("Failed to read line");

        if let Ok(id) = id_input.trim().parse::<u16>() {
            for task in &mut self.tasks {
                if task.id == id {
                    return Some(task);
                }
            }
            println!("No task found with ID: {}", id);
        } else {
            println!("Invalid ID entered. Please enter a numeric value.");
        }

        None
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
            println!("Priority (High/Medium/Low):");
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

    fn display_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks to display.");
        } else {
            // Print table headers
            println!("{:<5} {:<20} {:<30} {:<10} {:<10} {:<12}", "ID", "Name", "Description", "Priority", "Completed", "Due Date");
            println!("{}", "-".repeat(87));  // Adjust the total length based on the above lengths

            for task in &self.tasks {
                println!(
                    "{:<5} {:<20} {:<30} {:<10} {:<10} {:<12}",
                    task.id,
                    task.name,
                    task.description,
                    format!("{:?}", task.priority),
                    if task.completed { "Yes" } else { "No" },
                    task.due_date
                );
            }
        }
    }

    fn mark_task_as_completed(&mut self) {
        self.display_tasks();

        if let Some(task) = self.validate_id() {
            task.completed = true;
            println!("Task marked as completed.");
        }
    }

    fn edit_task(&mut self) {
        self.display_tasks();
        if let Some(task) = self.validate_id() {
            println!("Select the field you want to edit:");
            println!("1. Name");
            println!("2. Description");
            println!("3. Priority");
            println!("4. Due Date");

            let mut field_input = String::new();
            loop {
                io::stdin().read_line(&mut field_input).expect("Failed to read line");
                let choice = field_input.trim().parse::<u8>();

                match choice {
                    Ok(num) if num >= 1 && num <= 4 => {
                        match num {
                            1 => {
                                println!("Enter new name:");
                                let mut new_name = String::new();
                                io::stdin().read_line(&mut new_name).expect("Failed to read line");
                                task.name = new_name.trim().to_string();  // Update the task's name
                            },
                            2 => {
                                println!("Enter new description:");
                                let mut new_description = String::new();
                                io::stdin().read_line(&mut new_description).expect("Failed to read line");
                                task.description = new_description.trim().to_string();  // Update the description
                            },
                            3 => {
                                println!("Enter new priority (Low, Medium, High):");
                                let mut new_priority = String::new();
                                io::stdin().read_line(&mut new_priority).expect("Failed to read line");
                                task.priority = match new_priority.trim().to_lowercase().as_str() {
                                    "low" => Priority::Low,
                                    "medium" => Priority::Medium,
                                    "high" => Priority::High,
                                    _ => {
                                        println!("Invalid priority entered. Keeping previous.");
                                        continue;  // Ask again if invalid
                                    }
                                };
                            },
                            4 => {
                                println!("Enter new due date (YYYY-MM-DD):");
                                let mut new_date = String::new();
                                io::stdin().read_line(&mut new_date).expect("Failed to read line");
                                match chrono::NaiveDate::parse_from_str(new_date.trim(), "%Y-%m-%d") {
                                    Ok(date) => task.due_date = date,  // Update the due date
                                    Err(_) => {
                                        println!("Invalid date format. Please enter in YYYY-MM-DD format.");
                                        continue;  // Ask again if invalid
                                    }
                                }
                            },
                            _ => unreachable!(), // We already validate num to be 1-4, so this should not happen
                        }
                        println!("Task updated successfully.");
                        break;  // Exit the loop after successful update
                    },
                    _ => {
                        println!("Invalid choice. Please enter a number between 1 and 4:");
                        field_input.clear();
                    }
                }
            }
        }
    }

    fn remove_task(&mut self) {
        self.display_tasks();

        // First, find the task to get a reference and confirm its existence.
        if let Some(task) = self.validate_id() {
            // We now know the task exists. Find its index for removal.
            // This is necessary since validate_id doesn't provide the index.
            let task_id = task.id; // Store the ID of the task to remove.
            if let Some(index) = self.tasks.iter().position(|t| t.id == task_id) {
                self.tasks.remove(index); // Remove the task by index.
                println!("Task successfully removed.");
            } else {
                println!("Task not found.");
            }
        }
    }
}

fn get_user_input() -> u8 {
    loop {
        println!("-----------------------");
        println!("To-do List");
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
            2 => task_manager.display_tasks(),
            3 => task_manager.mark_task_as_completed(),
            4 => task_manager.edit_task(),
            5 => task_manager.remove_task(),
            6 => {
                println!("Exiting the program.");
                break;
            },
            _ => println!("Invalid option. Please try again."),
        }
    }
}
