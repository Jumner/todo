use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::task::Task;
use anyhow::Result;
use inquire::Select;

use super::List;

impl List {
    pub fn pick_task(&self) -> Result<Rc<RefCell<Task>>> {
        let mut map = HashMap::new();
        for item in self.tasks.values().cloned() {
            map.insert(item.borrow().name.clone(), item.clone());
        }
        let ans = Select::new("Select a Task", map.keys().cloned().collect())
            // .with_help_message("")
            .with_vim_mode(true)
            .prompt()
            .unwrap();
        return Ok(map.get(&ans).unwrap().clone());
    }
}
