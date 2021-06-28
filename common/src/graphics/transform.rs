use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub tx: f32,
    pub ty: f32,
    pub sx: f32,
    pub sy: f32,
}

impl Transform {
    pub fn identity() -> Self {
        Transform { tx: 0., ty: 0., sx: 1., sy: 1. }
    }

    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        (
            x * self.sx + self.tx,
            y * self.sy + self.ty,
        )
    }

    pub fn apply_reverse(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x - self.tx) / self.sx,
            (y - self.ty) / self.sy,
        )
    }

    pub fn scale(&self, width: f32, height: f32) -> (f32, f32) {
        (
            width * self.sx,
            height * self.sy
        )
    }

    pub fn scale_reverse(&self, width: f32, height: f32) -> (f32, f32) {
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