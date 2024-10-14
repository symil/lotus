use parsable::{parsable, ItemLocation};
use crate::program::EventCallbackStep;

#[parsable(name = r#""start" | "progress" | "end""#)]
pub enum ParsedEventCallbackEventStep {
    Start = "start",
    Progress = "progress",
    End = "end"
}

impl ParsedEventCallbackEventStep {
    pub fn process(&self) -> EventCallbackStep {
        match self {
            ParsedEventCallbackEventStep::Start => EventCallbackStep::Start,
            ParsedEventCallbackEventStep::Progress => EventCallbackStep::Progress,
            ParsedEventCallbackEventStep::End => EventCallbackStep::End,
        }
    }
}