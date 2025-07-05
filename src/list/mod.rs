use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Duration};
pub mod cli;
mod stress;

use crate::task::Task;
use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    time: Vec<Duration>,
    tasks: HashMap<usize, Rc<RefCell<Task>>>,
    id_counter: usize,
}

impl List {
    pub fn new(time: Vec<Duration>) -> Self {
        List {
            time,
            tasks: HashMap::new(),
            id_counter: 0,
        }
    }

    pub fn add_task(&mut self, task: Rc<RefCell<Task>>) -> Result<()> {
        task.borrow_mut().initialize(self.id_counter).unwrap();
        self.id_counter += 1;
        self.tasks.insert(task.borrow().id, task.clone());
        self.update_supertasks(task.clone());
        self.update_subtasks(task.clone());
        return Ok(());
    }

    pub fn remove_task(&mut self, task: Rc<RefCell<Task>>) -> Result<()> {
        self.tasks.remove(&task.borrow().id);
        // Break subtasks
        for subtask in task.borrow().subtasks.values().cloned() {
            subtask.borrow_mut().supertasks.remove(&task.borrow().id);
        }
        // Break supertasks
        for supertask in task.borrow().supertasks.iter() {
            self.tasks
                .get(supertask)
                .cloned()
                .unwrap()
                .borrow_mut()
                .subtasks
                .remove(&task.borrow().id);
        }
        return Ok(());
    }

    pub fn sort(&mut self) {
        println!(
            "{:?}",
            self.tasks
                .values()
                .cloned()
                .sorted()
                .collect::<Vec<Rc<RefCell<Task>>>>()
        );
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.time).unwrap();
        writeln!(f, "Tasks:").unwrap();
        for task in self.tasks.values().cloned() {
            write!(f, "{}", task.borrow()).unwrap();
        }
        write!(f, "")
    }
}
