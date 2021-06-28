use super::rect::Rect;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Zero,
    Real(f32),
    Virtual(f32),
    ScaledFromHeight(f32),
    ScaledFromWidth(f32)
}

impl Size {
    pub fn to_fixed(&self, rect: &Rect, virtual_to_real_ratio: f32) -> f32 {
        match self {
            Size::Zero => 0.0,
            Size::Real(value) => *value,
            Size::Virtual(value) => *value * virtual_to_real_ratio,
            Size::ScaledFromHeight(value) => rect.height * value,
            Size::ScaledFromWidth(value) => rect.width * value,
        }
    }
}