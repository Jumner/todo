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

    pub fn pick_task_list(&self) -> Result<Rc<RefCell<Task>>> {
        let map = self.get_name_map();
        let task = Select::new("Select a Task", get_tasks(map.clone(), |_| true).unwrap())
            // .with_help_message("")
            .with_vim_mode(true)
            .prompt()
            .unwrap();
        return Ok(map.get(&task).unwrap().clone());
    }

    pub fn pick_task_hierarchy(&self) -> Result<Rc<RefCell<Task>>> {
        let map = self.get_name_map();
        let mut root = Select::new(
            "Select a Task",
            get_tasks(map.clone(), |task| task.borrow().supertasks.is_empty()).unwrap(),
        )
        // .with_help_message("")
        .with_vim_mode(true)
        .prompt();
        let mut task = map.get(root.as_ref().unwrap()).unwrap().clone();
        loop {
            let name = if let Ok(name) = root {
                name
            } else {
                return Ok(task);
            };
            task = map.get(&name).unwrap().clone();
            if task.borrow().subtasks.len() == 0 {
                return Ok(task);
            }
            match Select::new("Search subtasks or select task", vec!["Continue", "Select"])
                // .with_help_message("")
                .with_vim_mode(true)
                .prompt()
                .unwrap()
            {
                "Continue" => {}
                "Select" => {
                    return Ok(task);
                }
                _ => {}
            }
            root = Select::new(
                "Select a Task",
                task.borrow().subtasks.keys().cloned().collect(),
            )
            // .with_help_message("")
            .with_vim_mode(true)
            .prompt();
        }
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
        let task = self.pick_task_hierarchy().unwrap();
        update_task(task.clone()).unwrap();
        // Set subtasks
        self.update_subtasks(task.clone());
        // Assign Parent
        self.update_supertasks(task.clone());

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
        let selected_subtasks = MultiSelect::new("Select subtasks", total_tasks.clone())
            // .with_help_message("")
            .with_vim_mode(true)
            .with_default(&current_subtasks)
            .with_help_message("Select subtasks")
            .prompt();
        let selected_subtasks = if let Ok(selected_subtasks) = selected_subtasks {
            selected_subtasks
        } else {
            return;
        };
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

    fn update_supertasks(&mut self, task: Rc<RefCell<Task>>) {
        // get list of children
        let mut children = HashSet::new();
        let mut stack = vec![task.borrow().name.clone()];
        while let Some(child) = stack.pop() {
            for subtask in self.tasks.get(&child).unwrap().borrow().subtasks.keys() {
                stack.push(subtask.clone());
                children.insert(subtask.clone());
            }
        }
        // Get list of tasks
        let map = self.get_name_map();
        let total_tasks = get_tasks(map.clone(), |other| {
            other.borrow().name != task.borrow().name
                && !children.contains(&other.borrow().name.clone())
        })
        .unwrap();
        let current_supertasks: Vec<usize> = total_tasks
            .iter()
            .enumerate()
            .filter_map(|(i, other)| {
                if task.borrow().supertasks.contains(other) {
                    return Some(i);
                }
                None
            })
            .collect();
        let selected_subtasks = MultiSelect::new("Select Supertasks", total_tasks.clone())
            // .with_help_message("")
            .with_vim_mode(true)
            .with_default(&current_supertasks)
            .with_help_message("Select supertasks")
            .prompt();
        let selected_supertasks = if let Ok(selected_supertasks) = selected_subtasks {
            selected_supertasks
        } else {
            return;
        };
        current_supertasks.iter().for_each(|&supertask| {
            if !selected_supertasks.contains(&total_tasks[supertask]) {
                let name = task.borrow().name.clone();
                self.tasks
                    .get(&total_tasks[supertask])
                    .unwrap()
                    .borrow_mut()
                    .remove_subtask(name);
            }
        });
        selected_supertasks.iter().for_each(|other| {
            if !task.borrow().supertasks.contains(other) {
                self.tasks
                    .get(other)
                    .unwrap()
                    .borrow_mut()
                    .add_subtask(task.clone());
            }
        });
    }
}
