use super::List;

impl List {
    pub fn total_stress(&self) -> f32 {
        self.tasks
            .keys()
            .filter_map(|&id| {
                if self.tasks.get(&id).unwrap().started() {
                    return Some(self.stress(id));
                }
                return None;
            })
            .sum()
    }
}
