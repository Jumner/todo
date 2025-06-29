use std::{cell::RefCell, rc::Rc};

use inquire::Select;

use crate::{list::List, task::cli::get_task};

pub fn main_menu(list: &mut List) {
    match Select::new(
        "Select Action",
        vec!["Add Task", "Remove Task", "Modify Task", "Complete Task"],
    )
    // .with_help_message("")
    .with_vim_mode(true)
    .prompt()
    .unwrap()
    {
        "Add Task" => {
            let task = get_task();
            list.add_task(Rc::new(RefCell::new(task))).unwrap();
        }
        "Modify Task" => {
            let task = list.pick_task().unwrap();
            list.modify_task(task).unwrap();
        }
        _ => {
            println!("Unknown Action");
        }
    }
}
