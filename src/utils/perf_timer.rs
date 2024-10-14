use std::{time::Instant, mem::take};

pub struct PerfTimer {
    start: Instant,
    items: Vec<Item>,
    current: Option<Item>
}

#[derive(Debug, Clone)]
struct Item {
    label: &'static str,
    start: Instant,
    duration: u128
}

impl PerfTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            items: vec![],
            current: None
        }
    }

    pub fn start(label: &'static str) -> Self {
        let mut timer = Self::new();

        timer.trigger(label);

        timer
    }

    pub fn stop(&mut self) {
        if let Some(mut item) = take(&mut self.current) {
            item.duration = item.start.elapsed().as_millis();
            self.items.push(item);
        }
    }

    pub fn trigger(&mut self, label: &'static str) {
        self.stop();

        if !label.is_empty() {
            self.current = Some(Item {
                label,
                start: Instant::now(),
                duration: 0,
            });
        }
    }

    pub fn to_string(&mut self, separator: &str, threshold: u128) -> String {
        self.stop();

        let mut items = self.items.clone();
        items.push(Item {
            label: "total",
            start: self.start,
            duration: self.start.elapsed().as_millis(),
        });

        let mut lines : Vec<String> = items.iter()
            .filter(|item| item.duration >= threshold)
            .map(|item| format!("{}: {}ms", &item.label, item.duration)).collect();

        lines.join(separator)
    }

    pub fn display(&mut self) {
        let string = self.to_string("\n", 0);
        println!("{}", string);
    }
}