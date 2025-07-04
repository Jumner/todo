use std::{cell::RefCell, rc::Rc};

use inquire::Select;

use crate::{
    list::List,
    task::{Task, cli::create_task},
};

pub fn main_menu(list: &mut List) {
    println!("Overall Stress {:.2}", list.stress());
    match Select::new(
        "Select Action",
        vec![
            "Add Task",
            "Remove Task",
            "Modify Task",
            "Complete Task",
            "View Task",
            "Priority List",
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
            let task = list.pick_task(|_| true).unwrap();
            list.remove_task(task).unwrap();
        }
        "Modify Task" => {
            let task = list.pick_task(|_| true).unwrap();
            list.modify_task(task).unwrap();
        }
        "Complete Task" => {
            let task = list
                .pick_task(|task: Rc<RefCell<Task>>| task.borrow().subtasks.len() == 0)
                .unwrap();
            if list.complete_task(task).is_err() {
                println!("Dependency not completed");
            }
        }
        "View Task" => {
            let task = list.pick_task(|_| true).unwrap();
            println!("{}", task.borrow());
        }
        "Priority List" => {
            let task = list.ordered_list().unwrap();
            println!("{}", task.borrow());
        }
        _ => {
            println!("Unknown Action");
        }
    }
}
