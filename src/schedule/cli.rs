use super::{Itinerary, Schedule, TimeBlock};
use crate::task::cli::get_time;
use chrono::{NaiveDate, NaiveTime, Weekday};
use inquire::{DateSelect, Select};

pub fn create_timeblock() -> TimeBlock {
    let mut timeblock = TimeBlock::new();
    timeblock.update();
    return timeblock;
}

impl TimeBlock {
    pub fn update(&mut self) {
        self.start = get_time(
            Some(self.start),
            String::from("Select the start time of the timeblock"),
            String::from("Enter the start time"),
        )
        .unwrap();
        self.end = get_time(
            Some(self.end),
            String::from("Select the end time of the timeblock"),
            String::from("Enter the end time"),
        )
        .unwrap();
    }
}

pub fn create_itinerary() -> Itinerary {
    let mut itinerary = Itinerary::new();
    itinerary.update();
    return itinerary;
}

impl Itinerary {
    pub fn update(&mut self) {
        loop {
            match Select::new(
                "Select Action",
                vec![
                    "Add Timeblock",
                    "Remove Timeblock",
                    "Modify Timeblock",
                    "Done",
                ],
            )
            .prompt()
            .unwrap()
            {
                "Add Timeblock" => {
                    self.create_timeblock();
                }
                "Remove Timeblock" => {
                    self.remove_timeblock();
                }
                "Modify Timeblock" => {
                    self.modify_timeblock();
                }
                "Done" => {
                    return;
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    fn create_timeblock(&mut self) {
        let timeblock = create_timeblock();
        self.timeblocks.insert(timeblock);
    }

    fn remove_timeblock(&mut self) {
        let timeblock = *self.select_timeblock();
        self.timeblocks.remove(&timeblock);
    }

    fn modify_timeblock(&mut self) {
        let timeblock = *self.select_timeblock();
        let mut timeblock = self.timeblocks.take(&timeblock).unwrap();
        let copy = timeblock.clone();
        timeblock.update();
        if let Err(err) = self.add_timeblock(timeblock) {
            println!("Couldn't update timeblock: {}", err);

            self.add_timeblock(copy).unwrap();
        }
    }

    fn select_timeblock(&self) -> &TimeBlock {
        Select::new("Select Timeblock", self.timeblocks.iter().collect())
            .prompt()
            .unwrap()
    }
}

impl Schedule {
    pub fn update(&mut self) {
        loop {
            match Select::new(
                "Select Action",
                vec!["Update Schedule", "Update Default Schedule", "Done"],
            )
            .prompt()
            .unwrap()
            {
                "Update Schedule" => {
                    let date = select_date();
                    if let Some(itinerary) = self.schedule.get_mut(&date) {
                        itinerary.update();
                        continue;
                    }
                    let itinerary = create_itinerary();
                    self.schedule.insert(date, itinerary);
                }
                "Update Default Schedule" => {
                    let day = select_day();
                    self.default_schedule.get_mut_itinerary(day).update();
                }
                "Done" => {
                    return;
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}

fn select_day() -> Weekday {
    Select::new(
        "Select Action",
        vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
    )
    .prompt()
    .unwrap()
}

fn select_date() -> NaiveDate {
    DateSelect::new("Select Date to Schedule")
        .with_help_message("Enter a date")
        .prompt()
        .unwrap()
}
