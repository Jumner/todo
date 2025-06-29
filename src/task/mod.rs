use anyhow::{Result, anyhow};
use chrono::{NaiveDateTime, TimeDelta};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub mod cli;
mod status;
pub use status::Status;

#[derive(Debug)]
pub struct Task {
    pub id: Option<usize>,
    pub name: String,
    description: String,
    status: Status,
    estimated_time: TimeDelta,
    estimated_value: usize,
    deadline: NaiveDateTime,
    subtasks: HashMap<String, Rc<RefCell<Task>>>,
}

impl Task {
    pub fn new(
        name: String,
        description: String,
        estimated_time: TimeDelta,
        estimated_value: usize,
        deadline: NaiveDateTime,
    ) -> Self {
        return Task {
            id: None,
            name,
            description,
            status: Status::INVALID,
            estimated_time,
            estimated_value,
            deadline,
            subtasks: HashMap::new(),
        };
    }

    pub fn initialize(&mut self, id: usize) -> Result<()> {
        if let Some(id) = self.id {
            return Err(anyhow!("ID is already initialized ({})", id));
        }
        self.id = Some(id);
        match self.status {
            Status::INVALID => self.status = Status::INCOMPLETE,
            status => return Err(anyhow!("Status is not invalid ({})", status)),
        };
        return Ok(());
    }

    pub fn declare_subtask(&mut self, task: Rc<RefCell<Task>>) {
        let name = task.borrow().name.clone();
        self.subtasks.insert(name, task);
    }

    pub fn cost(&self) -> f32 {
        return self.estimated_time.as_seconds_f32();
    }

    pub fn complete(&mut self) -> Result<()> {
        for task in self.subtasks.values().cloned() {
            match task.borrow().status {
                Status::COMPLETE => continue,
                _ => {
                    return Err(anyhow!(
                        "Error subtask \"{}\" is not complete",
                        task.borrow().name.as_str()
                    ));
                }
            }
        }
        self.status = Status::COMPLETE;
        Ok(())
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {:?}", self.id).unwrap();
        writeln!(f, "Name: {}", self.name).unwrap();
        writeln!(f, "Description: {}", self.description).unwrap();
        writeln!(f, "Status: {}", self.status).unwrap();
        writeln!(f, "Estimated Hours: {}", self.estimated_time.num_hours()).unwrap();
        writeln!(f, "Estimated Value: {}", self.estimated_value).unwrap();
        writeln!(f, "Deadline: {:?}", self.deadline).unwrap();
        for subtask in self.subtasks.keys() {
            writeln!(f, "Subtask: {}", subtask).unwrap();
        }
        write!(f, "")
    }
}

impl std::cmp::Eq for Task {}

impl std::cmp::PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.cost() == other.cost()
    }
}

impl std::cmp::PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl std::cmp::Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cost = self.cost();
        let other_cost = other.cost();
        if cost > other_cost {
            return std::cmp::Ordering::Greater;
        } else if cost < other_cost {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
