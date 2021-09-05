use serializable::Serializable;

#[derive(Debug, Default, Clone, Copy, Serializable)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn as_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn add(&self, p: Self) -> Self {
        Self {
            x: p.x + self.x,
            y: p.y + self.y,
        }
    }

    pub fn substract(&self, p: Self) -> Self {
        Self {
            x: p.x - self.x,
            y: p.y - self.y,
        }
    }

    pub fn multiply(&self, ratio: f64) -> Self {
        Self {
            x: self.x * ratio,
            y: self.y * ratio,
        }
    }

    pub fn divide(&self, ratio: f64) -> Self {
        Self {
            x: self.x / ratio,
            y: self.y / ratio,
        }
    }

    pub fn vector_to(&self, other: Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn vector_to_normalized(&self, other: Self) -> Self {
        self.vector_to(other).normalize()
    }

    pub fn normalize(&self) -> Self {
        self.divide(self.length())
    }

    pub fn distance_to(&self, other: Self) -> f64 {
        self.vector_to(other).length()
    }
}