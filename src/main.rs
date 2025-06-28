use std::time::{Duration, Instant};

use todo::{List, Task};

fn main() {
    let mut task = Task::new(
        4,
        String::from("Name"),
        String::from("Description"),
        Duration::from_secs(7),
        10,
        Instant::now(),
    );
    let subtask = Task::new(
        5,
        String::from("Subtask"),
        String::from("Subdescription"),
        Duration::from_secs(5),
        10,
        Instant::now(),
    );
    task.declare_subtask(5);
    let mut list = List::new(vec![Duration::from_secs(10)]);
    list.add_task(task);
    list.add_task(subtask);
    println!("{}", list);
    list.sort();
    println!("{}", list);
}
