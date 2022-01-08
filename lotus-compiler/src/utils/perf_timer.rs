use std::{time::Instant, mem::take};

pub struct PerfTimer {
    items: Vec<Item>,
    current: Option<Item>
}

struct Item {
    label: &'static str,
    start: Instant,
    duration: u128
}

impl PerfTimer {
    pub fn new() -> Self {
        Self {
            items: vec![],
            current: None
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut item) = take(&mut self.current) {
            item.duration = item.start.elapsed().as_millis();
            self.items.push(item);
        }
    }

    pub fn trigger(&mut self, label: &'static str) {
        self.stop();
        self.current = Some(Item {
            label,
            start: Instant::now(),
            duration: 0,
        });
    }

    pub fn display(&mut self) {
        self.stop();

        for item in &self.items {
            println!("{}: {}ms", item.label, item.duration);
        }
    }
}