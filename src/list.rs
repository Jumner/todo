use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::Task;

#[derive(Debug)]
pub struct List {
    time: Vec<Duration>,
    tasks: Vec<Rc<RefCell<Task>>>,
}

impl List {
    pub fn new(time: Vec<Duration>) -> Self {
        List {
            time,
            tasks: Vec::new(),
        }
    }
    pub fn add_task(&mut self, task: Rc<RefCell<Task>>) {
        self.tasks.push(task);
    }

    pub fn sort(&mut self) {
        self.tasks.sort();
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.time).unwrap();
        writeln!(f, "Tasks:").unwrap();
        for task in self.tasks.iter() {
            write!(f, "{}", task.borrow()).unwrap();
        }
        write!(f, "")
    }
}
