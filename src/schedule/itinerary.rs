use anyhow::{Result, anyhow};
use chrono::{NaiveTime, TimeDelta};

#[derive(Debug, Clone)]
pub struct Itinerary {
    blocks: Vec<TimeBlock>,
}

impl Itinerary {
    pub fn new() -> Self {
        Itinerary { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, block: TimeBlock) -> Result<()> {
        if self.overlaps(&block) {
            return Err(anyhow!("Block overlaps with itinerary"));
        }
        self.blocks.push(block);
        Ok(())
    }

    fn overlaps(&self, other_block: &TimeBlock) -> bool {
        for block in self.blocks.iter() {
            if block.overlaps(other_block) {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimeBlock {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl TimeBlock {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Self {
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
