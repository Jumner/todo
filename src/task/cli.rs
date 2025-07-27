use anyhow::Result;
use chrono::{self, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use inquire::{CustomType, DateSelect, Text};

use crate::task::Task;

pub fn create_task() -> Task {
    let name = get_name(None).unwrap();
    let description = get_description(None).unwrap();
    let estimated_time = get_estimated_time(None).unwrap();

    let estimated_stress = get_estimated_stress(None).ok();
    let start = get_datetime(None, true).ok();
    let deadline = get_datetime(None, false).ok();
    return Task::new(
        name,
        description,
        TimeDelta::try_hours(estimated_time as i64).unwrap(),
        estimated_stress,
        start,
        deadline,
    );
}

impl Task {
    pub fn update_task(&mut self) {
        let name = get_name(Some(self.name.clone())).unwrap();
        let description = get_description(Some(self.description.clone())).unwrap();
        let estimated_time =
            get_estimated_time(Some(self.estimated_time.num_hours() as usize)).unwrap();

        let estimated_stress = get_estimated_stress(self.estimated_stress).ok();
        let start = get_datetime(self.start, true).ok();
        let deadline = get_datetime(self.deadline, false).ok();
        self.name = name;
        self.description = description;
        self.estimated_time = TimeDelta::try_hours(estimated_time as i64).unwrap();
        self.estimated_stress = estimated_stress;
        self.start = start;
        self.deadline = deadline;
    }
}

pub fn get_time(
    default: Option<NaiveTime>,
    message: String,
    help_message: String,
) -> Result<NaiveTime> {
    let mut time = CustomType::new(message.as_str())
        .with_parser(&|i| NaiveTime::parse_from_str(i, "%H:%M:%S").map_err(|_e| ()))
        .with_help_message(help_message.as_str())
        .with_error_message("WRONG");
    if let Some(default) = default {
        time = time.with_default(default);
    }
    let time = time.prompt()?;
    return Ok(time);
}

fn get_date(default: Option<NaiveDate>, message: String) -> Result<NaiveDate> {
    let mut date = DateSelect::new(message.as_str());
    if let Some(default) = default {
        date = date.with_default(default);
    }
    let date = date.prompt()?;
    return Ok(date);
}

fn get_datetime(default: Option<NaiveDateTime>, start: bool) -> Result<NaiveDateTime> {
    let (guess_time, date_message, time_message, time_help_message) = if start {
        (
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            String::from("Select the start date of the task"),
            String::from("Select the time the task starts"),
            String::from("Enter the start time"),
        )
    } else {
        (
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            String::from("Select the due date of the task"),
            String::from("Select the time the task is due"),
            String::from("Enter the due time"),
        )
    };
    let (default_date, default_time) = if let Some(default) = default {
        (Some(default.date()), Some(default.time()))
    } else {
        (None, Some(guess_time))
    };

    let date = get_date(default_date, date_message)?;
    let time = get_time(default_time, time_message, time_help_message).unwrap_or(guess_time);
    return Ok(date.and_time(time));
}

fn get_estimated_stress(default: Option<f32>) -> Result<f32> {
    let mut estimated_stress = CustomType::new("Estimated Additional Stress")
        .with_formatter(&|i: f32| format!("${i}"))
        .with_error_message("Please type a valid number")
        .with_help_message("Enter the stress of the task");
    if let Some(default) = default {
        estimated_stress = estimated_stress.with_default(default);
    }
    let estimated_stress = estimated_stress.prompt()?;
    return Ok(estimated_stress);
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
