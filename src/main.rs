use std::{cell::RefCell, rc::Rc, time::Duration};
use todo::{list::List, task::cli::get_task};

fn main() {
    let task = Rc::new(RefCell::new(get_task()));
    let subtask = Rc::new(RefCell::new(get_task()));
    task.borrow_mut().declare_subtask(subtask.clone());
    let mut list = List::new(vec![Duration::from_secs(10)]);
    list.add_task(task).unwrap();
    list.add_task(subtask).unwrap();
    println!("{}", list);
    list.sort();
    println!("{}", list);
    list.update_task().unwrap();
    // println!("{}", list.pick_task().unwrap().borrow());
    // println!("{:?}", list.pick_tasks().unwrap());
}
