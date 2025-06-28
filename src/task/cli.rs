use chrono::{self, NaiveTime};
use inquire::{CustomType, DateSelect, Text};
use std::time::Duration;

use crate::task::Task;

pub fn get_task() -> Task {
    let name = Text::new("Task Name")
        .with_help_message("Enter the name of the new task")
        .prompt()
        .unwrap();
    let description = Text::new("Task Description")
        .with_help_message("Describe the task")
        .prompt()
        .unwrap();
    let estimated_time = Duration::from_secs(10);
    let estimated_value = CustomType::new("Estimated Value")
        .with_formatter(&|i: usize| format!("${i}"))
        .with_error_message("Please type a valid number")
        .with_help_message("Enter the value of the task")
        .prompt()
        .unwrap();
    let date = DateSelect::new("Select the due date of the task")
        .prompt()
        .unwrap();
    let time = CustomType::new("Select the time the task is due")
        .with_parser(&|i| NaiveTime::parse_from_str(i, "%H:%M:%S").map_err(|_e| ()))
        .with_help_message("Enter the due time")
        .with_error_message("WRONG")
        .with_default(NaiveTime::from_hms_opt(23, 59, 59).unwrap())
        .prompt()
        .unwrap();
    return Task::new(
        name,
        description,
        estimated_time,
        estimated_value,
        date.and_time(time),
    );
}
