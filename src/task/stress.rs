use chrono::TimeDelta;

use crate::list::List;

fn hours(time: &TimeDelta) -> f32 {
    return time.as_seconds_f32() / 3600.0;
}

impl List {
    fn effective_stress(&self, id: usize) -> f32 {
        let parents = self.get_all_parents(id);
        return parents
            .iter()
            .map(|task| {
                self.tasks
                    .get(task)
                    .unwrap()
                    .estimated_stress
                    .unwrap_or(0.0)
            })
            .sum();
    }

    pub fn effective_time(&self, id: usize) -> TimeDelta {
        let children = self.get_all_children(id);
        return children
            .iter()
            .map(|task| self.tasks.get(task).unwrap().estimated_time)
            .sum();
    }

    fn base_stess() -> f32 {
        // Each task has a bit of stress associated with its sheer existance.
        return 0.5; // Hr/Days (I'm calling it a Schuette)
    }

    fn hours_til_started(&self, id: usize) -> Option<f32> {
        // Number of hours until we need to start a task
        // TODO Take now and compute when the earliest date it can be completed is.
        // Find time from earliest completion date to the due date
        let deadline = self.tasks.get(&id).unwrap().deadline;
        let time_til_due = if let Some(deadline) = deadline {
            self.schedule.time_until(deadline) - self.effective_time(id)
        } else {
            return None;
        };
        return Some(hours(&time_til_due));
    }

    fn crunch_stress(&self, id: usize) -> f32 {
        // This is the hard one...
        // The stress from a task due x hours from now is:
        let hours = self.hours_til_started(id).unwrap_or(120.0).max(0.0);
        let f =
            |x: f32| 22.5 * (-1.05 * x).exp() + 2.0 * (-0.05 * x).exp() + 0.5 * (-0.003 * x).exp();
        return f(hours);
    }

    pub fn stress(&self, id: usize) -> f32 {
        if !self.tasks.get(&id).unwrap().started() {
            return 0.0;
        }
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

        let hours = hours(&self.effective_time(id));
        let stress =
            self.crunch_stress(id) * (List::base_stess() + self.effective_stress(id)) / hours;
        return stress.max(child_stress);
    }
}
