use super::Task;

impl Task {
    fn base_stess(&self) -> f32 {
        // Each task has a bit of stress associated with its sheer existance.
        return 0.5; // Hr/Days (I'm calling it a Schuette)
    }

    fn crunch_stress(&self) -> f32 {
        // This is the hard one...
        // The stress from a task due x hours from now is:
        // 22.5e^{-1.05x}+2e^{-0.05x}+0.5e^{-0.003x} Schuettes
        0.0
    }

    fn value_stress(&self) -> f32 {
        // How many dollars is worth 1 Schuette? Maybe like 50
        return 1.0 / 50.0 * self.estimated_value as f32;
    }

    pub fn stress(&self) -> f32 {
        let hours = self.estimated_time.as_seconds_f32() / 3600.0;
        return (self.base_stess() + self.crunch_stress() + self.value_stress()) / hours;
    }
}
