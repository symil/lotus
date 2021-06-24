use super::rect::Rect;

pub enum Size {
    Zero,
    Fixed(f32),
    ScaledFromHeight(f32),
    ScaledFromWidth(f32)
}

impl Size {
    pub fn to_fixed(&self, rect: &Rect) -> f32 {
        match self {
            Size::Zero => 0.0,
            Size::Fixed(value) => *value,
            Size::ScaledFromHeight(value) => rect.height * value,
            Size::ScaledFromWidth(value) => rect.width * value,
        }
    }
}