#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCallbackStep {
    Start,
    Progress,
    End
}