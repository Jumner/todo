use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Duration};
pub mod cli;

use crate::task::Task;
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
pub struct List {
    time: Vec<Duration>,
    tasks: HashMap<String, Rc<RefCell<Task>>>,
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
        self.tasks.insert(task.borrow().name.clone(), task.clone());
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
