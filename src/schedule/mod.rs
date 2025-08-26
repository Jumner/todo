pub mod cli;
mod default_schedule;
mod itinerary;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Weekday};
use default_schedule::DefaultSchedule;
pub use itinerary::{Itinerary, TimeBlock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    schedule: HashMap<NaiveDate, Itinerary>,
    default_schedule: DefaultSchedule,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            schedule: HashMap::new(),
            default_schedule: DefaultSchedule::new(),
        }
    }

    pub fn earliest_complete(
        &self,
        mut time: TimeDelta,
        task_start: Option<NaiveDateTime>,
    ) -> NaiveDateTime {
        println!("SCHEDULE");
        let mut start = Local::now().naive_local();
        if let Some(task_start) = task_start {
            start = start.max(task_start);
        }
        let mut start_time = start.time();
        for date in start.date().iter_days() {
            let itinerary = self.get_itinerary(date);
            (start_time, time) = itinerary.earliest_complete(start_time, time);
            if time == TimeDelta::zero() {
                return date.and_time(start_time);
            }
            start_time = NaiveTime::MIN;
        }
        unreachable!("Task not completable?");
    }

    pub fn time_until(&self, datetime: NaiveDateTime) -> TimeDelta {
        let now = Local::now().naive_local();
        self.time_between(now, datetime)
    }

    fn time_between(&self, start: NaiveDateTime, end: NaiveDateTime) -> TimeDelta {
        let mut time = TimeDelta::zero();
        let mut start_time = start.time();
        for date in start.date().iter_days() {
            if date > end.date() {
                break;
            }
            let end_time = if date == end.date() {
                end.time()
            } else {
                NaiveTime::from_hms_opt(23, 59, 59).unwrap()
            };
            let itinerary = self.get_itinerary(date);
            time += itinerary.time_between(start_time, end_time);
            start_time = NaiveTime::MIN;
        }
        time
    }

    pub fn get_itinerary(&self, date: NaiveDate) -> &Itinerary {
        if let Some(itinerary) = self.schedule.get(&date) {
            return itinerary;
        }
        return self.default_schedule.get_itinerary(date.weekday());
    }

    pub fn get_mut_itinerary(&mut self, date: NaiveDate) -> &mut Itinerary {
        if let Some(itinerary) = self.schedule.get_mut(&date) {
            return itinerary;
        }
        return self.default_schedule.get_mut_itinerary(date.weekday());
    }

    pub fn set_itinerary(&mut self, date: NaiveDate, itinerary: Itinerary) {
        self.schedule.insert(date, itinerary);
    }

    pub fn set_default_itinerary(&mut self, weekday: Weekday, itinerary: Itinerary) {
        self.default_schedule.set_itinerary(weekday, itinerary);
    }

    pub fn clean(&mut self) {
        let today = Local::now().naive_local().date();
        self.schedule.retain(|&date, _| date >= today);
    }
}
