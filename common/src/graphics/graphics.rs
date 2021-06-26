use wasm_bindgen::prelude::*;
use as_js_string_macro::*;
use super::{color::Color, size::Size, transform::Transform};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle,
    Rectangle,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Font {
    Arial
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextHorizontalAlign {
    Left,
    Center,
    Right
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextVerticalAlign {
    Top,
    Middle,
    Bottom
}

#[as_js_string]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Cursor {
    Default,
    Pointer,
    Text,
}

#[derive(Debug, Clone)]
pub struct Graphics {
    pub transform: Transform,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: f32,
    pub height: f32,
    pub aspect_ratio: f32,
    pub scale: f32,
    pub angle: f32,
    pub border_color: Color,
    pub border_alpha: f32,
    pub border_width: Size,
    pub border_radius: Size,
    pub background_color: Color,
    pub background_alpha: f32,
    pub overlay_color: Color,
    pub overlay_alpha: f32,
    pub image_url: Option<String>,
    pub text: Option<String>,
    pub text_font: Font,
    pub text_size: Size,
    pub text_color: Color,
    pub text_margin: Size,
    pub text_max_width: Option<Size>,
    pub text_max_height: Option<Size>,
    pub text_background_color: Color,
    pub text_border_color: Color,
    pub text_horizontal_align: TextHorizontalAlign,
    pub text_vertical_align: TextVerticalAlign,
    pub text_bold: bool,
    pub text_italic: bool,
    pub cursor: Cursor,
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            x: 0.,
            y: 0.,
            z: 0.,
            offset_x: 0.,
            offset_y: 0.,
            width: 0.,
            height: 0.,
            aspect_ratio: 0.,
            scale: 1.,
            angle: 0.,
            border_color: Color::transparent(),
            border_alpha: 1.,
            border_width: Size::Zero,
            border_radius: Size::Zero,
            background_color: Color::transparent(),
            background_alpha: 1.,
            overlay_color: Color::transparent(),
            overlay_alpha: 1.,
            image_url: None,
            text: None,
            text_font: Font::Arial,
            text_size: Size::Zero,
            text_color: Color::transparent(),
            text_margin: Size::Zero,
            text_max_width: None,
            text_max_height: None,
            text_background_color: Color::transparent(),
            text_border_color: Color::transparent(),
            text_horizontal_align: TextHorizontalAlign::Center,
            text_vertical_align: TextVerticalAlign::Middle,
            text_bold: false,
            text_italic: false,
            cursor: Cursor::Default,
        }
    }
}