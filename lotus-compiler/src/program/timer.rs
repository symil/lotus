use std::time::Instant;
use indexmap::IndexMap;
use super::ProgramStep;

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

    pub fn start(&mut self, step: ProgramStep) {
        self.start_timestamps.insert(step, Instant::now());
    }

    pub fn stop(&mut self, step: ProgramStep) -> f64 {
        let elapsed = self.start_timestamps.get(&step).unwrap().elapsed().as_secs_f64();

        self.durations.insert(step, elapsed);

        elapsed
    }

    pub fn consume(self) -> Vec<(ProgramStep, f64)> {
        let mut result = vec![];
        let mut total_duration = 0;

        for (step, duration) in &self.durations {
            result.push((*step, *duration));
        }

        result
    }
}