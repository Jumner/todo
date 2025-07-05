use chrono::Local;

use crate::list::List;

impl List {
    fn base_stess(&self, _id: usize) -> f32 {
        // Each task has a bit of stress associated with its sheer existance.
        return 0.5; // Hr/Days (I'm calling it a Schuette)
    }

    fn hours_til_due(&self, id: usize) -> Option<f32> {
        let now = Local::now().naive_local();
        let deadline = self.tasks.get(&id).unwrap().deadline;
        let time_til_due = if let Some(deadline) = deadline {
            deadline.signed_duration_since(now)
        } else {
            return None;
        };
        return Some(time_til_due.as_seconds_f32() / 3600.0);
    }

    fn crunch_stress(&self, id: usize) -> f32 {
        // This is the hard one...
        // The stress from a task due x hours from now is:
        let hours = self.hours_til_due(id).unwrap_or(120.0).max(0.0);
        let f =
            |x: f32| 22.5 * (-1.05 * x).exp() + 2.0 * (-0.05 * x).exp() + 0.5 * (-0.003 * x).exp();
        return f(hours);
    }

    fn value_stress(&self, id: usize) -> Option<f32> {
        // How many dollars is worth 1 Schuette? Maybe like 50
        let value = if let Some(value) = self.tasks.get(&id).unwrap().estimated_value {
            value as f32
        } else {
            return None;
        };
        return Some(1.0 / 50.0 * value);
    }

    pub fn stress(&self, id: usize) -> f32 {
        let child_stress = self
            .tasks
            .get(&id)
            .unwrap()
            .subtasks
            .iter()
            .cloned()
            .map(|x| self.stress(x))
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        let hours = self.tasks.get(&id).unwrap().estimated_time.as_seconds_f32() / 3600.0;
        let stress = self.crunch_stress(id)
            * (self.base_stess(id) + self.value_stress(id).unwrap_or(0.0))
            / hours;
        return stress.max(child_stress);
    }
}
