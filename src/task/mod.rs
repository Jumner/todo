use anyhow::Result;
use chrono::{NaiveDateTime, TimeDelta};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub mod cli;
mod stress;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub name: String,
    description: String,
    estimated_time: TimeDelta,
    estimated_value: Option<usize>,
    pub start: Option<NaiveDateTime>,
    deadline: Option<NaiveDateTime>,
    pub subtasks: HashSet<usize>,
    pub supertasks: HashSet<usize>,
}

impl Task {
    pub fn new(
        name: String,
        description: String,
        estimated_time: TimeDelta,
        estimated_value: Option<usize>,
        start: Option<NaiveDateTime>,
        deadline: Option<NaiveDateTime>,
    ) -> Self {
        return Task {
            id: 0,
            name,
            description,
            estimated_time,
            estimated_value,
            start,
            deadline,
            subtasks: HashSet::new(),
            supertasks: HashSet::new(),
        };
    }

    pub fn initialize(&mut self, id: usize) -> Result<()> {
        self.id = id;
        return Ok(());
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {:?}", self.id).unwrap();
        writeln!(f, "Name: {}", self.name).unwrap();
        writeln!(f, "Description: {}", self.description).unwrap();
        writeln!(f, "Estimated Hours: {}", self.estimated_time.num_hours()).unwrap();
        if let Some(value) = self.estimated_value {
            writeln!(f, "Estimated Value: {}", value).unwrap();
        }
        if let Some(deadline) = self.deadline {
            writeln!(f, "Deadline: {:?}", deadline).unwrap();
        }
        write!(f, "")
    }
}
