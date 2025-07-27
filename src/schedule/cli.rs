use chrono::NaiveTime;

use crate::task::cli::get_time;

use super::TimeBlock;

pub fn create_timeblock() -> TimeBlock {
    let start = get_time(
        NaiveTime::from_hms_opt(0, 0, 0),
        String::from("Select the start time of the timeblock"),
        String::from("Enter the start time"),
    )
    .unwrap();
    let end = get_time(
        NaiveTime::from_hms_opt(0, 0, 0),
        String::from("Select the end time of the timeblock"),
        String::from("Enter the end time"),
    )
    .unwrap();
    return TimeBlock::new(start, end);
}
