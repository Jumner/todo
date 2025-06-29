use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::task::{Task, cli::update_task};
use anyhow::Result;
use inquire::{MultiSelect, Select};

use super::List;

fn get_tasks<F: FnMut(Rc<RefCell<Task>>) -> bool>(
    map: HashMap<String, Rc<RefCell<Task>>>,
    mut filter: F,
) -> Result<Vec<String>> {
    Ok(map
        .into_iter()
        .filter_map(|(name, task)| {
            if filter(task) {
                return Some(name);
            }
            None
        })
        .collect())
}

impl List {
    fn get_name_map(&self) -> HashMap<String, Rc<RefCell<Task>>> {
        let mut map = HashMap::new();
        for item in self.tasks.values().cloned() {
            map.insert(item.borrow().name.clone(), item.clone());
        }
        return map;
    }

    pub fn pick_task(&self) -> Result<Rc<RefCell<Task>>> {
        let map = self.get_name_map();
        let task = Select::new("Select a Task", get_tasks(map.clone(), |_| true).unwrap())
            // .with_help_message("")
            .with_vim_mode(true)
            .prompt()
            .unwrap();
        return Ok(map.get(&task).unwrap().clone());
    }

    pub fn pick_tasks(&self) -> Result<Vec<Rc<RefCell<Task>>>> {
        let map = self.get_name_map();
        let tasks = MultiSelect::new("Select Tasks", get_tasks(map.clone(), |_| true).unwrap())
            // .with_help_message("")
            .with_vim_mode(true)
            .prompt()
            .unwrap();
        return Ok(tasks
            .into_iter()
            .map(|task| map.get(&task).unwrap().clone())
            .collect());
    }

    pub fn update_task(&mut self) -> Result<()> {
        let task = self.pick_task().unwrap();
        update_task(task.clone()).unwrap();
        // Set subtasks
        // Get list of tasks
        let map = self.get_name_map();
        let tasks = get_tasks(map.clone(), |other| {
            other.borrow().name != task.borrow().name
        })
        .unwrap();
        let default: Vec<usize> = tasks
            .iter()
            .enumerate()
            .filter_map(|(i, other)| {
                if task.borrow().subtasks.contains_key(other) {
                    return Some(i);
                }
                None
            })
            .collect();
        let task_select = MultiSelect::new("Select Tasks", tasks)
            // .with_help_message("")
            .with_vim_mode(true)
            .with_default(&default)
            .with_help_message("Select subtasks")
            .prompt()
            .unwrap();
        println!("{:?}", task_select);
        // Assign Parent
        Ok(())
    }
}
