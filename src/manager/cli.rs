use std::{cell::RefCell, rc::Rc};

use inquire::Select;

use crate::{list::List, task::cli::create_task};

pub fn main_menu(list: &mut List) {
    match Select::new(
        "Select Action",
        vec![
            "Add Task",
            "Remove Task",
            "Modify Task",
            "Complete Task",
            "View Task",
        ],
    )
    // .with_help_message("")
    .with_vim_mode(true)
    .prompt()
    .unwrap()
    {
        "Add Task" => {
            let task = create_task();
            list.add_task(Rc::new(RefCell::new(task))).unwrap();
        }
        "Remove Task" => {
            let task = list.pick_task().unwrap();
            list.remove_task(task).unwrap();
        }
        "Modify Task" => {
            let task = list.pick_task().unwrap();
            list.modify_task(task).unwrap();
        }
        "Complete Task" => {
            let task = list.pick_task().unwrap();
            if list.complete_task(task).is_err() {
                println!("Dependency not completed");
            }
        }
        _ => {
            println!("Unknown Action");
        }
    }
}
