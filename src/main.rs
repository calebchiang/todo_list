mod lib;
fn main() {
    let mut task_manager = lib::TaskManager::new();

    loop {
        let user_input = lib::get_user_input();
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
