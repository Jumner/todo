use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use chrono::{self, NaiveDate, NaiveTime, TimeDelta};
use inquire::{CustomType, DateSelect, Text};

use crate::task::Task;

pub fn create_task() -> Task {
    let name = get_name(None).unwrap();
    let description = get_description(None).unwrap();
    let estimated_time = get_estimated_time(None).unwrap();

    let estimated_value = get_estimated_value(None).unwrap();
    let date = get_date(None).unwrap();
    let time = get_time(Some(NaiveTime::from_hms_opt(23, 59, 59).unwrap())).unwrap();
    return Task::new(
        name,
        description,
        TimeDelta::try_hours(estimated_time as i64).unwrap(),
        estimated_value,
        date.and_time(time),
    );
}

fn get_time(default: Option<NaiveTime>) -> Result<NaiveTime> {
    let mut time = CustomType::new("Select the time the task is due")
        .with_parser(&|i| NaiveTime::parse_from_str(i, "%H:%M:%S").map_err(|_e| ()))
        .with_help_message("Enter the due time")
        .with_error_message("WRONG");
    if let Some(default) = default {
        time = time.with_default(default);
    }
    let time = time.prompt().unwrap();
    return Ok(time);
}

fn get_date(default: Option<NaiveDate>) -> Result<NaiveDate> {
    let mut date = DateSelect::new("Select the due date of the task");
    if let Some(default) = default {
        date = date.with_default(default);
    }
    let date = date.prompt().unwrap();
    return Ok(date);
}

fn get_estimated_value(default: Option<usize>) -> Result<usize> {
    let mut estimated_value = CustomType::new("Estimated Value")
        .with_formatter(&|i: usize| format!("${i}"))
        .with_error_message("Please type a valid number")
        .with_help_message("Enter the value of the task");
    if let Some(default) = default {
        estimated_value = estimated_value.with_default(default);
    }
    let estimated_value = estimated_value.prompt().unwrap();
    return Ok(estimated_value);
}

fn get_estimated_time(default: Option<usize>) -> Result<usize> {
    let mut estimated_time = CustomType::new("Estimated Hours Required")
        .with_formatter(&|i: usize| format!("{i} Hours"))
        .with_error_message("Please type a valid number")
        .with_help_message("Enter the number of hours required");
    if let Some(default) = default {
        estimated_time = estimated_time.with_default(default);
    }
    let estimated_time = estimated_time.prompt().unwrap();
    return Ok(estimated_time);
}

fn get_name(default: Option<String>) -> Result<String> {
    let mut name = Text::new("Task Name").with_help_message("Enter the name of the new task");
    if let Some(default) = default.as_ref() {
        name = name.with_default(default);
    }
    let name = name.prompt().unwrap();
    return Ok(name);
}

fn get_description(default: Option<String>) -> Result<String> {
    let mut description = Text::new("Task Description").with_help_message("Describe the task");
    if let Some(default) = default.as_ref() {
        description = description.with_default(default);
    }
    let description = description.prompt().unwrap();
    return Ok(description);
}

pub fn update_task(task: Rc<RefCell<Task>>) -> Result<()> {
    let mut task = task.borrow_mut();
    let name = get_name(Some(task.name.clone())).unwrap();
    let description = get_description(Some(task.description.clone())).unwrap();
    let estimated_time =
        get_estimated_time(Some(task.estimated_time.num_hours() as usize)).unwrap();

    let estimated_value = get_estimated_value(Some(task.estimated_value)).unwrap();
    let date = get_date(Some(task.deadline.date())).unwrap();
    let time = get_time(Some(task.deadline.time())).unwrap();

    task.name = name;
    task.description = description;
    task.estimated_time = TimeDelta::try_hours(estimated_time as i64).unwrap();
    task.estimated_value = estimated_value;
    task.deadline = date.and_time(time);
    Ok(())
}
