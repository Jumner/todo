use std::time::{Duration, Instant};

use inquire::Text;

use crate::task::Task;

pub fn get_task() -> Task {
    let name = Text::new("Task Name: ").prompt().unwrap();
    let description = String::from("temp");
    let estimated_time = Duration::from_secs(10);
    let estimated_value = 5;
    let deadline = Instant::now();
    return Task::new(name, description, estimated_time, estimated_value, deadline);
}
