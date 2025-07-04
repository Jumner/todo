use chrono::Local;

use super::Task;

impl Task {
    fn base_stess(&self) -> f32 {
        // Each task has a bit of stress associated with its sheer existance.
        return 0.5; // Hr/Days (I'm calling it a Schuette)
    }

    fn hours_til_due(&self) -> f32 {
        let now = Local::now().naive_local();
        let time_til_due = self.deadline.signed_duration_since(now);
        return time_til_due.as_seconds_f32() / 3600.0;
    }

    fn crunch_stress(&self) -> f32 {
        // This is the hard one...
        // The stress from a task due x hours from now is:
        let hours = self.hours_til_due().max(0.0);
        let f =
            |x: f32| 22.5 * (-1.05 * x).exp() + 2.0 * (-0.05 * x).exp() + 0.5 * (-0.003 * x).exp();
        return f(hours);
    }

    fn value_stress(&self) -> f32 {
        // How many dollars is worth 1 Schuette? Maybe like 50
        return 1.0 / 50.0 * self.estimated_value as f32;
    }

    pub fn stress(&self) -> f32 {
        let child_stress = self
            .subtasks
            .values()
            .cloned()
            .map(|x| x.borrow().stress())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        let hours = self.estimated_time.as_seconds_f32() / 3600.0;
        let stress = (self.base_stess() + self.value_stress()) / hours + self.crunch_stress();
        return stress.max(child_stress);
    }
}
