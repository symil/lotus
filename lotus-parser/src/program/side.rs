use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Side {
    Left,
    Right
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Left => write!(f, "left"),
            Side::Right => write!(f, "right"),
        }
    }
}