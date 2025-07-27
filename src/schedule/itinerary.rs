use std::collections::HashSet;

use anyhow::{Result, anyhow};
use chrono::{NaiveTime, TimeDelta, Timelike};

#[derive(Debug, Clone)]
pub struct Itinerary {
    pub timeblocks: HashSet<TimeBlock>,
}

impl Itinerary {
    pub fn new() -> Self {
        Itinerary {
            timeblocks: HashSet::new(),
        }
    }

    pub fn add_timeblock(&mut self, block: TimeBlock) -> Result<()> {
        if self.overlaps(&block) {
            return Err(anyhow!("Block overlaps with itinerary"));
        }
        self.timeblocks.insert(block);
        Ok(())
    }

    fn overlaps(&self, other_block: &TimeBlock) -> bool {
        for block in self.timeblocks.iter() {
            if block.overlaps(other_block) {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TimeBlock {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl std::fmt::Display for TimeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:02}:{:02}:{:02} - {:02}:{:02}:{:02}",
            self.start.hour(),
            self.start.minute(),
            self.start.second(),
            self.end.hour(),
            self.end.minute(),
            self.end.second()
        )
    }
}

impl TimeBlock {
    pub fn new() -> Self {
        TimeBlock {
            start: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        }
    }

    pub fn from_start_end(start: NaiveTime, end: NaiveTime) -> Self {
        TimeBlock { start, end }
    }

    pub fn duration(&self) -> TimeDelta {
        self.end.signed_duration_since(self.start)
    }

    fn overlaps(&self, other_block: &TimeBlock) -> bool {
        if self.start >= other_block.start && self.start < other_block.end {
            return true;
        }
        if other_block.start >= self.start && other_block.start < self.end {
            return true;
        }
        return false;
    }
}
