use wasm_bindgen::prelude::*;
use as_js_string_macro::*;
use super::{color::Color, rect::Rect, size::Size};

macro_rules! make_enum {
    ($name:ident : $default:ident $(, $field:ident)*) => {
        #[as_js_string(lowercase)]
        #[wasm_bindgen]
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $default,
            $( $field , )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default
            }
        }
    }
}

make_enum! { Shape : Rectangle, Circle, Line }
make_enum! { Font : Arial }
make_enum! { TextHorizontalAlign : Center, Left, Right }
make_enum! { TextVerticalAlign : Middle, Top, Bottom }
make_enum! { Cursor : Default, Pointer, Text }

#[derive(Debug, Clone)]
pub struct Graphics {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: f32,
    pub height: f32,
    pub shape: Shape,
    pub aspect_ratio: Option<f32>,
    pub scale: f32,
    pub angle: f32,
    pub border_radius: Size,
    pub border_width: Size,
    pub border_dash_length: Size,
    pub border_gap_length: Size,
    pub border_color: Color,
    pub border_alpha: f32,
    pub background_color: Color,
    pub background_alpha: f32,
    pub overlay_color: Color,
    pub overlay_alpha: f32,
    pub image_url: Option<String>,
    pub image_scale: f32,
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
    pub detectable: bool,
    pub cursor: Cursor,
}

impl Graphics {
    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            offset_x: 0.,
            offset_y: 0.,
            width: 0.,
            height: 0.,
            shape: Shape::Rectangle,
            aspect_ratio: None,
            scale: 1.,
            angle: 0.,
            border_color: Color::transparent(),
            border_alpha: 1.,
            border_width: Size::Zero,
            border_radius: Size::Zero,
            border_dash_length: Size::Zero,
            border_gap_length: Size::Zero,
            background_color: Color::transparent(),
            background_alpha: 1.,
            overlay_color: Color::transparent(),
            overlay_alpha: 1.,
            image_url: None,
            image_scale: 1.,
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
            detectable: true,
            cursor: Cursor::Default,
        }
    }
}

#[macro_export]
macro_rules! graphics {
    (rect: $rect:expr $(, $name:ident : $value:expr)*) => {
        {
            let rect = $rect;

            Graphics {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                $( $name: $value, )*
                ..Graphics::default()
            }
        }
    }
}