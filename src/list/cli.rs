use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

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
        self.update_subtasks(task);
        // Assign Parent

        // TODO filter out direct or indirect children
        Ok(())
    }

    fn update_subtasks(&mut self, task: Rc<RefCell<Task>>) {
        // get list of parents
        let mut parents = HashSet::new();
        let mut stack = vec![task.borrow().name.clone()];
        while let Some(parent) = stack.pop() {
            for supertask in self.tasks.get(&parent).unwrap().borrow().supertasks.iter() {
                stack.push(supertask.clone());
                parents.insert(supertask.clone());
            }
        }
        // Get list of tasks
        let map = self.get_name_map();
        let total_tasks = get_tasks(map.clone(), |other| {
            other.borrow().name != task.borrow().name
                && !parents.contains(&other.borrow().name.clone())
        })
        .unwrap();
        let current_subtasks: Vec<usize> = total_tasks
            .iter()
            .enumerate()
            .filter_map(|(i, other)| {
                if task.borrow().subtasks.contains_key(other) {
                    return Some(i);
                }
                None
            })
            .collect();
        let selected_subtasks = MultiSelect::new("Select Tasks", total_tasks.clone())
            // .with_help_message("")
            .with_vim_mode(true)
            .with_default(&current_subtasks)
            .with_help_message("Select subtasks")
            .prompt();
        if let Ok(selected_subtasks) = selected_subtasks {
            current_subtasks.iter().for_each(|&subtask| {
                if !selected_subtasks.contains(&total_tasks[subtask]) {
                    task.borrow_mut()
                        .remove_subtask(total_tasks[subtask].clone());
                }
            });
            selected_subtasks.iter().for_each(|other| {
                if !task.borrow().subtasks.contains_key(other) {
                    task.borrow_mut()
                        .add_subtask(self.tasks.get(other).unwrap().clone());
                }
            });
        }
    }
}
