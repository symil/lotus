pub struct Transform {
    pub tx: f32,
    pub ty: f32,
    pub sx: f32,
    pub sy: f32,
    pub r: f32
}

impl Transform {
    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x * self.sx + self.tx) * self.r,
            (y * self.sy + self.ty) * self.r,
        )
    }

    pub fn apply_reverse(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x / self.r - self.tx) / self.sx,
            (y / self.r - self.ty) / self.sy,
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            tx: 0.,
            ty: 0.,
            sx: 1.,
            sy: 1.,
            r: 1.
        }
    }
}