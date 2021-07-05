use super::rect::Rect;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Zero,
    Real(f64),
    Virtual(f64),
    ScaledFromHeight(f64),
    ScaledFromWidth(f64),
    ScaledFromMin(f64),
}

impl Size {
    pub fn to_fixed(&self, rect: &Rect, virtual_to_real_ratio: f64) -> f64 {
        match self {
            Size::Zero => 0.0,
            Size::Real(value) => *value,
            Size::Virtual(value) => *value * virtual_to_real_ratio,
            Size::ScaledFromHeight(value) => rect.height * value,
            Size::ScaledFromWidth(value) => rect.width * value,
            Size::ScaledFromMin(value) => min(rect.width, rect.height) * value
        }
    }
}

fn min(a: f64, b: f64) -> f64 {
    match a < b {
        true => a,
        false => b
    }
}