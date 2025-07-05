use super::List;

impl List {
    pub fn total_stress(&self) -> f32 {
        self.tasks.keys().map(|&id| self.stress(id)).sum()
    }
}
