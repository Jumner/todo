use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use chrono::{self, NaiveDate, NaiveTime, TimeDelta};
use inquire::{CustomType, DateSelect, Text};

use crate::task::Task;

pub fn get_task() -> Task {
    let name = get_name().unwrap();
    let description = get_description().unwrap();
    let estimated_time = get_estimated_time().unwrap();

    let estimated_value = get_estimated_value().unwrap();
    let date = get_date().unwrap();
    let time = get_time().unwrap();
    return Task::new(
        name,
        description,
        TimeDelta::try_hours(estimated_time as i64).unwrap(),
        estimated_value,
        date.and_time(time),
    );
}

fn get_time() -> Result<NaiveTime> {
    let time = CustomType::new("Select the time the task is due")
        .with_parser(&|i| NaiveTime::parse_from_str(i, "%H:%M:%S").map_err(|_e| ()))
        .with_help_message("Enter the due time")
        .with_error_message("WRONG")
        .with_default(NaiveTime::from_hms_opt(23, 59, 59).unwrap())
        .prompt()
        .unwrap();
    return Ok(time);
}

fn get_date() -> Result<NaiveDate> {
    let date = DateSelect::new("Select the due date of the task")
        .prompt()
        .unwrap();
    return Ok(date);
}

fn get_estimated_value() -> Result<usize> {
    let estimated_value = CustomType::new("Estimated Value")
        .with_formatter(&|i: usize| format!("${i}"))
        .with_error_message("Please type a valid number")
        .with_help_message("Enter the value of the task")
        .prompt()
        .unwrap();
    return Ok(estimated_value);
}

fn get_estimated_time() -> Result<usize> {
    let estimated_time = CustomType::new("Estimated Hours Required")
        .with_formatter(&|i: usize| format!("{i} Hours"))
        .with_error_message("Please type a valid number")
        .with_help_message("Enter the number of hours required")
        .prompt()
        .unwrap();
    return Ok(estimated_time);
}

fn get_name() -> Result<String> {
    let name = Text::new("Task Name")
        .with_help_message("Enter the name of the new task")
        .prompt()
        .unwrap();
    return Ok(name);
}

fn get_description() -> Result<String> {
    let description = Text::new("Task Description")
        .with_help_message("Describe the task")
        .prompt()
        .unwrap();
    return Ok(description);
}

pub fn update_task(task: Rc<RefCell<Task>>) {
    let name = get_name().unwrap();
    let description = get_description().unwrap();
    let estimated_time = get_estimated_time().unwrap();

    let estimated_value = get_estimated_value().unwrap();
    let date = get_date().unwrap();
    let time = get_time().unwrap();
    let mut task = task.borrow_mut();
    task.name = name;
    task.description = description;
    task.estimated_time = TimeDelta::try_hours(estimated_time as i64).unwrap();
    task.estimated_value = estimated_value;
    task.deadline = date.and_time(time);
}
