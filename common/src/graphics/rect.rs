use wasm_bindgen::prelude::*;
use super::{transform::Transform};
use crate::serialization::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serializable)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Rect {
    fn default() -> Self {
        Rect::new(0., 0., 0., 0.)
    }
}

impl Rect {
    pub fn x1(&self) -> f32 {
        self.x - self.width / 2.
    }

    pub fn y1(&self) -> f32 {
        self.y - self.height / 2.
    }

    pub fn x2(&self) -> f32 {
        self.x + self.width / 2.
    }

    pub fn y2(&self) -> f32 {
        self.y + self.height / 2.
    }

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_top_left(x1: f32, y1: f32, width: f32, height: f32) -> Self {
        Self {
            x: x1 + width / 2.0,
            y: y1 + height / 2.0,
            width,
            height,
        }
    }

    pub fn from_corners(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            x: (x1 + x2) / 2.0,
            y: (y1 + y2) / 2.0,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn from_size(width: f32, height: f32) -> Self {
        Self {
            x: width / 2.0,
            y: height / 2.0,
            width,
            height,
        }
    }

    pub fn round(&self) -> Self {
        Self::from_corners(self.x1().round(), self.y1().round(), self.x2().round(), self.y2().round())
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x > self.x1() && x < self.x2() && y > self.y1() && y < self.y2()
    }

    pub fn translate(&self, x: f32, y: f32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            width: self.width,
            height: self.height
        }
    }

    pub fn scale(&self, ratio: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            width: self.width * ratio,
            height: self.height * ratio
        }
    }

    pub fn pad(&self, width: f32, height: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            width: self.width + width,
            height: self.height + height
        }
    }

    pub fn strip(&self, width: f32, height: f32) -> Self {
        self.pad(-width, -height)
    }

    pub fn strip_margin(&self, margin: f32) -> Self {
        let to_stip = margin * 2.;

        self.strip(to_stip, to_stip)
    }

    pub fn pad_to_match_aspect_ratio(&self, aspect_ratio: Option<f32>) -> Self {
        match aspect_ratio {
            None => self.clone(),
            Some(ratio) => {
                let width_from_height = self.height * ratio;
                let height_from_width = self.width / ratio;
                let mut width_to_pad = 0.;
                let mut height_to_pad = 0.;

                if self.width < width_from_height {
                    width_to_pad = width_from_height - self.width;
                } else {
                    height_to_pad = height_from_width - self.height;
                }

                self.pad(width_to_pad, height_to_pad)
            }
        }
    }

    pub fn strip_to_match_aspect_ratio(&self, aspect_ratio: Option<f32>) -> Self {
        match aspect_ratio {
            None => self.clone(),
            Some(ratio) => {
                let width_from_height = self.height * ratio;
                let height_from_width = self.width / ratio;
                let mut width_to_strip = 0.;
                let mut height_to_strip = 0.;

                if self.width > width_from_height {
                    width_to_strip = self.width - width_from_height;
                } else {
                    height_to_strip = self.height - height_from_width;
                }

                self.strip(width_to_strip, height_to_strip)
            }
        }
    }

    pub fn multiply(&self, ratio: f32) -> Self {
        Self {
            x: self.x * ratio,
            y: self.y * ratio,
            width: self.width * ratio,
            height: self.height * ratio
        }
    }

    pub fn split_horizontally(&self, left_width: f32) -> (Self, Self) {
        let right_width = self.width - left_width;

        (
            Self::from_top_left(self.x1(), self.y1(), left_width, self.height),
            Self::from_top_left(self.x2() - right_width, self.y1(), right_width, self.height),
        )
    }

    pub fn split_vertically(&self, top_height: f32) -> (Self, Self) {
        let bottom_height = self.width - top_height;

        (
            Self::from_top_left(self.x1(), self.y1(), self.width, top_height),
            Self::from_top_left(self.x1(), self.y2() - bottom_height, self.width, bottom_height),
        )
    }

    pub fn symmetry(&self, x: f32, y: f32) -> Self {
        Self {
            x: 2.0 * x - self.x,
            y: 2.0 * y - self.y,
            width: self.width,
            height: self.height
        }
    }

    pub fn transform(&self, transform: &Transform) -> Self {
        let (x, y) = transform.apply(self.x, self.y);
        let (width, height) = transform.scale(self.width, self.height);

        Self { x, y, width, height }
    }
}