use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::task::Task;
use anyhow::Result;
use inquire::{MultiSelect, Select};

use super::List;

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
        let task = Select::new("Select a Task", map.keys().cloned().collect())
            // .with_help_message("")
            .with_vim_mode(true)
            .prompt()
            .unwrap();
        return Ok(map.get(&task).unwrap().clone());
    }

    pub fn pick_tasks(&self) -> Result<Vec<Rc<RefCell<Task>>>> {
        let map = self.get_name_map();
        let tasks = MultiSelect::new("Select Tasks", map.keys().cloned().collect())
            // .with_help_message("")
            .with_vim_mode(true)
            .prompt()
            .unwrap();
        return Ok(tasks
            .into_iter()
            .map(|task| map.get(&task).unwrap().clone())
            .collect());
    }
}
