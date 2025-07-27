mod default_schedule;
mod itinerary;
use chrono::{Datelike, NaiveDate, Weekday};
use default_schedule::DefaultSchedule;
pub use itinerary::{Itinerary, TimeBlock};
use std::collections::HashMap;

#[derive(Debug, Clone)]
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

    pub fn get_itinerary(&self, date: NaiveDate) -> &Itinerary {
        if let Some(itinerary) = self.schedule.get(&date) {
            return itinerary;
        }
        return self.default_schedule.get_itinerary(date.weekday());
    }

    pub fn set_itinerary(&mut self, date: NaiveDate, itinerary: Itinerary) {
        self.schedule.insert(date, itinerary);
    }

    pub fn set_default_itinerary(&mut self, weekday: Weekday, itinerary: Itinerary) {
        self.default_schedule.set_itinerary(weekday, itinerary);
    }
}
