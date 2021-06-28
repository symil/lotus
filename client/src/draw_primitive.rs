use wasm_bindgen::prelude::*;
use lotus_common::graphics::{color::Color, graphics::{Font, Shape, TextHorizontalAlign, TextVerticalAlign}, rect::Rect, transform::Transform};

pub type StringId = i32;

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct DrawPrimitive {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub angle: f32,
    pub shape: Shape,
    pub border_radius: f32,
    pub border_color: Color,
    pub border_width: f32,
    pub border_dash_length: f32,
    pub border_gap_length: f32,
    pub background_color: Color,
    pub overlay_color: Color,
    pub image_url: Option<StringId>,
    pub image_width: f32,
    pub image_height: f32,
    pub text: Option<StringId>,
    pub text_font: Font,
    pub text_size: f32,
    pub text_color: Color,
    pub text_margin: f32,
    pub text_max_width: f32,
    pub text_max_height: f32,
    pub text_background_color: Color,
    pub text_border_color: Color,
    pub text_horizontal_align: TextHorizontalAlign,
    pub text_vertical_align: TextVerticalAlign,
    pub text_bold: bool,
    pub text_italic: bool,
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