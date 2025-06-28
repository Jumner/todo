mod list;
mod task;
use std::time::{Duration, Instant};

pub use list::List;
pub use task::Task;

pub fn forgor() {
    Task::new(
        5,
        String::from("Name"),
        String::from("Description"),
        Duration::from_secs(5),
        10,
        Instant::now(),
    );
    println!("I forgor");
}
