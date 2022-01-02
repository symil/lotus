use std::time::Instant;
use indexmap::IndexMap;
use super::ProgramStep;

#[derive(Debug, Default)]
pub struct Timer {
    durations: IndexMap<ProgramStep, f64>,
    start_timestamps: IndexMap<ProgramStep, Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            durations: IndexMap::new(),
            start_timestamps: IndexMap::new(),
        }
    }

    pub fn time<F : FnOnce()>(&mut self, step: ProgramStep, callback: F) -> f64 {
        self.start(step);
        callback();
        self.stop(step);
        
        *self.durations.get(&step).unwrap()
    }

    pub fn start(&mut self, step: ProgramStep) {
        self.start_timestamps.insert(step, Instant::now());
    }

    pub fn stop(&mut self, step: ProgramStep) -> f64 {
        let elapsed = self.start_timestamps.get(&step).unwrap().elapsed().as_secs_f64();

        self.durations.insert(step, elapsed);

        elapsed
    }

    pub fn get_total_duration(&self) -> f64 {
        let mut total = 0f64;

        for duration in self.durations.values() {
            total += *duration;
        }

        total
    }

    pub fn get_all_durations(&self) -> Vec<(ProgramStep, f64)> {
        let mut result = vec![];
        let mut total_duration = 0;

        for (step, duration) in &self.durations {
            result.push((*step, *duration));
        }

        result
    }
}