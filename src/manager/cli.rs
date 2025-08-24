use anyhow::Result;
use inquire::Select;

use crate::{
    list::List,
    task::{Task, cli::create_task},
};

pub fn main_menu(list: &mut List) -> Result<()> {
    println!("Overall Stress {:.2}", list.total_stress());
    match Select::new(
        "Select Action",
        vec![
            "Add Task",
            "Modify Task",
            "Complete Task",
            "View Task",
            "Update Schedule",
            "Update Default Schedule",
        ],
    )
    // .with_help_message("")
    .prompt()?
    {
        "Add Task" => {
            let task = create_task();
            list.add_task(task);
        }
        "Modify Task" => {
            let task = list.pick_task(|_| true);
            list.modify_task(task);
        }
        "Complete Task" => {
            let task = list.pick_task(|task: &Task| task.started() && task.subtasks.len() == 0);
            if list.complete_task(task).is_err() {
                println!("Dependency not completed");
            }
        }
        "View Task" => {
            let task = list.pick_task(|_| true);
            println!(
                "{}Earliest Completion: {}",
                list.tasks.get(&task).unwrap(),
                list.schedule.earliest_complete(list.effective_time(task))
            );
        }
        "Update Schedule" => {
            list.schedule.update();
        }
        "Update Default Schedule" => {
            list.schedule.update_default();
        }
        _ => {
            println!("Unknown Action");
        }
    }
    Ok(())
}
