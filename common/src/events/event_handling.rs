#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventHandling {
    Propagate,
    Intercept
}