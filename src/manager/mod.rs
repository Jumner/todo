pub mod cli;
use std::{cell::RefCell, rc::Rc, time::Duration};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use cli::main_menu;

use crate::{list::List, task::Task};

pub fn spin() {
    let task = Rc::new(RefCell::new(Task::new(
        String::from("Main Task"),
        String::from("Description of main"),
        TimeDelta::hours(5),
        10,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 6, 29).unwrap(),
            NaiveTime::from_hms_opt(11, 10, 9).unwrap(),
        ),
    )));
    let subtask = Rc::new(RefCell::new(Task::new(
        String::from("Sub Task"),
        String::from("Description of sub"),
        TimeDelta::hours(4),
        9,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 7, 4).unwrap(),
            NaiveTime::from_hms_opt(10, 9, 8).unwrap(),
        ),
    )));
    let subsubtask = Rc::new(RefCell::new(Task::new(
        String::from("Sub Sub Task"),
        String::from("Description of sub sub"),
        TimeDelta::hours(3),
        8,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 7, 15).unwrap(),
            NaiveTime::from_hms_opt(9, 8, 7).unwrap(),
        ),
    )));
    let other_subtask = Rc::new(RefCell::new(Task::new(
        String::from("Other Sub Task"),
        String::from("Description of sub"),
        TimeDelta::hours(4),
        9,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 7, 4).unwrap(),
            NaiveTime::from_hms_opt(10, 9, 8).unwrap(),
        ),
    )));
    let other_subsubtask = Rc::new(RefCell::new(Task::new(
        String::from("Other Sub Sub Task"),
        String::from("Description of sub sub"),
        TimeDelta::hours(3),
        8,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 7, 15).unwrap(),
            NaiveTime::from_hms_opt(9, 8, 7).unwrap(),
        ),
    )));
    let mut list = List::new(vec![Duration::from_secs(10)]);
    list.add_task(task.clone()).unwrap();
    list.add_task(subtask.clone()).unwrap();
    list.add_task(subsubtask.clone()).unwrap();
    list.add_task(other_subtask.clone()).unwrap();
    list.add_task(other_subsubtask.clone()).unwrap();
    loop {
        main_menu(&mut list);
    }
}
