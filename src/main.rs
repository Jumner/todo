use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use todo::{List, Task};

fn main() {
    let task = Rc::new(RefCell::new(Task::new(
        4,
        String::from("Name"),
        String::from("Description"),
        Duration::from_secs(7),
        10,
        Instant::now(),
    )));
    let subtask = Rc::new(RefCell::new(Task::new(
        5,
        String::from("Subtask"),
        String::from("Subdescription"),
        Duration::from_secs(5),
        10,
        Instant::now(),
    )));
    task.borrow_mut().declare_subtask(subtask.clone());
    let mut list = List::new(vec![Duration::from_secs(10)]);
    list.add_task(task);
    list.add_task(subtask);
    println!("{}", list);
    list.sort();
    println!("{}", list);
}
