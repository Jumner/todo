use std::collections::{HashMap, HashSet};
pub mod cli;
mod stress;

use crate::{schedule::Schedule, task::Task};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub tasks: HashMap<usize, Task>,
    id_counter: usize,
    pub schedule: Schedule,
}

impl List {
    pub fn new() -> Self {
        List {
            tasks: HashMap::new(),
            id_counter: 0,
            schedule: Schedule::new(),
        }
    }

    pub fn add_task(&mut self, mut task: Task) {
        task.initialize(self.id_counter).unwrap();
        self.id_counter += 1;
        let id = task.id;
        self.tasks.insert(id, task);
        self.update_supertasks(id);
        self.update_subtasks(id);
    }

    pub fn remove_task(&mut self, id: usize) {
        let task = self.tasks.remove(&id).unwrap();
        for subtask in task.subtasks.iter() {
            self.tasks.get_mut(subtask).unwrap().supertasks.remove(&id);
        }
        for supertask in task.supertasks.iter() {
            self.tasks.get_mut(supertask).unwrap().subtasks.remove(&id);
        }
    }

    pub fn add_subtask(&mut self, id: usize, subtask: usize) {
        self.tasks.get_mut(&id).unwrap().subtasks.insert(subtask);
        self.tasks.get_mut(&subtask).unwrap().supertasks.insert(id);
    }

    pub fn remove_subtask(&mut self, id: usize, subtask: usize) {
        self.tasks.get_mut(&id).unwrap().subtasks.remove(&subtask);
        self.tasks.get_mut(&subtask).unwrap().supertasks.remove(&id);
    }

    pub fn add_supertask(&mut self, id: usize, supertask: usize) {
        self.add_subtask(supertask, id);
    }

    pub fn remove_supertask(&mut self, id: usize, supertask: usize) {
        self.remove_subtask(supertask, id);
    }

    pub fn get_all_parents(&self, id: usize) -> HashSet<usize> {
        let mut parents = HashSet::from([id]);
        let mut stack = vec![id];
        while let Some(parent) = stack.pop() {
            for &supertask in self.tasks.get(&parent).unwrap().supertasks.iter() {
                stack.push(supertask);
                parents.insert(supertask);
            }
        }
        parents
    }

    pub fn get_all_children(&self, id: usize) -> HashSet<usize> {
        let mut children = HashSet::from([id]);
        let mut stack = vec![id];
        while let Some(child) = stack.pop() {
            for &subtask in self.tasks.get(&child).unwrap().subtasks.iter() {
                stack.push(subtask);
                children.insert(subtask);
            }
        }
        children
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tasks:").unwrap();
        for task in self.tasks.values() {
            write!(f, "{}", task).unwrap();
        }
        write!(f, "")
    }
}
