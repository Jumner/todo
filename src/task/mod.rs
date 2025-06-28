use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;
mod status;
pub use status::Status;

#[derive(Debug)]
pub struct Task {
    id: usize,
    name: String,
    description: String,
    status: Status,
    estimated_time: Duration,
    estimated_value: usize,
    deadline: Instant,
    subtasks: Vec<Rc<RefCell<Task>>>,
}

impl Task {
    pub fn new(
        id: usize,
        name: String,
        description: String,
        estimated_time: Duration,
        estimated_value: usize,
        deadline: Instant,
    ) -> Self {
        return Task {
            id,
            name,
            description,
            status: Status::INCOMPLETE,
            estimated_time,
            estimated_value,
            deadline,
            subtasks: vec![],
        };
    }

    pub fn declare_subtask(&mut self, task: Rc<RefCell<Task>>) {
        self.subtasks.push(task);
    }

    pub fn cost(&self) -> f32 {
        return self.estimated_time.as_secs() as f32;
    }

    pub fn complete(&mut self) {
        self.status = Status::COMPLETE;
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {}", self.id).unwrap();
        writeln!(f, "Name: {}", self.name).unwrap();
        writeln!(f, "Description: {}", self.description).unwrap();
        writeln!(f, "Status: {}", self.status).unwrap();
        writeln!(f, "Estimated Time: {:?}", self.estimated_time).unwrap();
        writeln!(f, "Estimated Value: {}", self.estimated_value).unwrap();
        writeln!(f, "Deadline: {:?}", self.deadline).unwrap();
        for subtask in self.subtasks.iter() {
            writeln!(f, "Subtask: {}", subtask.borrow().name).unwrap();
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
