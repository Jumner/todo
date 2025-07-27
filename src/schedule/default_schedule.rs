use crate::schedule::Itinerary;
use chrono::Weekday;

#[derive(Debug, Clone)]
pub struct DefaultSchedule {
    week: [Itinerary; 7],
}

impl DefaultSchedule {
    pub fn new() -> Self {
        DefaultSchedule {
            week: std::array::from_fn(|_| Itinerary::new()),
        }
    }

    pub fn get_itinerary(&self, day: Weekday) -> &Itinerary {
        &self.week[day.num_days_from_monday() as usize]
    }

    pub fn get_mut_itinerary(&mut self, day: Weekday) -> &mut Itinerary {
        &mut self.week[day.num_days_from_monday() as usize]
    }

    pub fn set_itinerary(&mut self, weekday: Weekday, itinerary: Itinerary) {
        self.week[weekday.num_days_from_monday() as usize] = itinerary;
    }
}
