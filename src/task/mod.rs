use anyhow::Result;
use chrono::{Local, NaiveDateTime, TimeDelta};
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
    estimated_stress: Option<f32>,
    start: Option<NaiveDateTime>,
    deadline: Option<NaiveDateTime>,
    pub subtasks: HashSet<usize>,
    pub supertasks: HashSet<usize>,
}

impl Task {
    pub fn new(
        name: String,
        description: String,
        estimated_time: TimeDelta,
        estimated_stress: Option<f32>,
        start: Option<NaiveDateTime>,
        deadline: Option<NaiveDateTime>,
    ) -> Self {
        return Task {
            id: 0,
            name,
            description,
            estimated_time,
            estimated_stress,
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
    pub fn started(&self) -> bool {
        let now = Local::now().naive_local();
        let start = if let Some(start) = self.start {
            start
        } else {
            return true;
        };
        return now.signed_duration_since(start).as_seconds_f32() > 0.0;
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {:?}", self.id).unwrap();
        writeln!(f, "Name: {}", self.name).unwrap();
        writeln!(f, "Description: {}", self.description).unwrap();
        writeln!(f, "Estimated Hours: {}", self.estimated_time.num_hours()).unwrap();
        if let Some(stress) = self.estimated_stress {
            writeln!(f, "Estimated Additional Stress: {}", stress).unwrap();
        }
        if let Some(start) = self.start {
            writeln!(f, "Start: {:?}", start).unwrap();
        }
        if let Some(deadline) = self.deadline {
            writeln!(f, "Deadline: {:?}", deadline).unwrap();
        }
        write!(f, "")
    }
}
