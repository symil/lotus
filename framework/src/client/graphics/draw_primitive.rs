use wasm_bindgen::prelude::*;

use crate::{Color, Font, Rect, Shape, TextHorizontalAlign, TextVerticalAlign};

pub type StringId = i32;

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct DrawPrimitive {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
    pub shape: Shape,
    pub border_radius: f64,
    pub border_color: Color,
    pub border_width: f64,
    pub border_dash_length: f64,
    pub border_gap_length: f64,
    pub background_color: Color,
    pub overlay_color: Color,
    pub image_url: Option<StringId>,
    pub image_width: f64,
    pub image_height: f64,
    pub text: Option<StringId>,
    pub text_font: Font,
    pub text_size: f64,
    pub text_color: Color,
    pub text_margin: f64,
    pub text_max_width: f64,
    pub text_max_height: f64,
    pub text_background_color: Color,
    pub text_border_color: Color,
    pub text_horizontal_align: TextHorizontalAlign,
    pub text_vertical_align: TextVerticalAlign,
    pub text_bold: bool,
    pub text_italic: bool,
    pub text_cursor_index: f64,
}

impl DrawPrimitive {
    pub fn from_rect(rect: &Rect) -> Self {
        let mut value = Self::default();

        value.set_rect(rect);

        value
    }

    pub fn set_rect(&mut self, rect: &Rect) {
        self.x = rect.x;
        self.y = rect.y;
        self.width = rect.width;
        self.height = rect.height;
    }
}