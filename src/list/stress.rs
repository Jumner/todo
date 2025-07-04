use super::List;

impl List {
    pub fn stress(&self) -> f32 {
        self.tasks.values().map(|task| task.borrow().stress()).sum()
    }
}
