use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub tx: f64,
    pub ty: f64,
    pub sx: f64,
    pub sy: f64,
}

impl Transform {
    pub fn identity() -> Self {
        Transform { tx: 0., ty: 0., sx: 1., sy: 1. }
    }

    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (
            x * self.sx + self.tx,
            y * self.sy + self.ty,
        )
    }

    pub fn apply_reverse(&self, x: f64, y: f64) -> (f64, f64) {
        (
            (x - self.tx) / self.sx,
            (y - self.ty) / self.sy,
        )
    }

    pub fn scale(&self, width: f64, height: f64) -> (f64, f64) {
        (
            width * self.sx,
            height * self.sy
        )
    }

    pub fn scale_reverse(&self, width: f64, height: f64) -> (f64, f64) {
        (
            width / self.sx,
            height / self.sy
        )
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            tx: other.tx + other.sx * self.tx,
            ty: other.ty + other.sy * self.ty,
            sx: other.sx * self.sx,
            sy: other.sx * self.sy,
        }
    }

    pub fn divide(&self, other: &Self) -> Self {
        Self {
            tx: (self.tx - other.tx) / other.sx,
            ty: (self.ty - other.ty) / other.sy,
            sx: self.sx / other.sx,
            sy: self.sy / other.sy,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}