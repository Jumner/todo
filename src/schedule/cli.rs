use super::{Itinerary, TimeBlock};
use crate::task::cli::get_time;
use inquire::Select;

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
                    self.add_timeblock();
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

    fn add_timeblock(&mut self) {
        let timeblock = create_timeblock();
        self.blocks.insert(timeblock);
    }

    fn remove_timeblock(&mut self) {
        let timeblock = *self.select_timeblock();
        self.blocks.remove(&timeblock);
    }

    fn modify_timeblock(&mut self) {
        let timeblock = *self.select_timeblock();
        let mut timeblock = self.blocks.take(&timeblock).unwrap();
        let copy = timeblock.clone();
        timeblock.update();
        if let Err(err) = self.add_block(timeblock) {
            println!("Couldn't update timeblock: {}", err);

            self.add_block(copy).unwrap();
        }
    }

    fn select_timeblock(&self) -> &TimeBlock {
        Select::new("Select Timeblock", self.blocks.iter().collect())
            .prompt()
            .unwrap()
    }
}
