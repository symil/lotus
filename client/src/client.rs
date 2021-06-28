use std::mem::replace;

use lotus_common::{graphics::{graphics::Graphics, rect::Rect, size::Size, transform::Transform}, traits::{player::Player, view::View}, view_context::ViewContext};

use crate::{draw_primitive::DrawPrimitive, js::Js};

#[derive(Debug)]
pub struct Client<P : Player, V : View<P>> {
    virtual_width: f32,
    virtual_height: f32,
    virtual_to_real_ratio: f32,
    player: Option<P>,
    cursor_x: f32,
    cursor_y: f32,
    hovered: Option<V>,
}

impl<P : Player, V : View<P>> Client<P, V> {
    pub fn new(virtual_width: f32, virtual_height: f32) -> Self {
        Self {
            virtual_width,
            virtual_height,
            virtual_to_real_ratio: 0.,
            player: None,
            cursor_x: 0.,
            cursor_y: 0.,
            hovered: None,
        }
    }

    pub fn start(&mut self) {
        let aspect_ratio = self.virtual_width / self.virtual_height;

        Js::set_window_aspect_ratio(aspect_ratio);
    }

    pub fn update(&mut self) {
        let (real_width, _real_height) = Js::get_window_size();

        self.virtual_to_real_ratio = real_width / self.virtual_width;

        while let Some(player) = Js::poll_message::<P>() {
            self.player = Some(player);
        }

        while let Some(event) = Js::poll_event() {
            if let Some(_) = event.window {
                Js::clear_renderer_cache();
            } else if let Some(mouse_event) = event.mouse {
                self.cursor_x = mouse_event.x;
                self.cursor_y = mouse_event.y;
            } else if let Some(_keyboard_event) = event.keyboard {

            }
        }

        if self.player.is_some() {
            let rect = Rect::from_size(self.virtual_width, self.virtual_height);
            let root = V::root(rect);

            self.hovered = self.render_view(root, Transform::identity()).and_then(|(view, _)| Some(view));
        }
    }

    fn render_view(&mut self, view: V, current_transform: Transform) -> Option<(V, f32)> {
        let context = &ViewContext {
            player: self.player.as_ref().unwrap(),
            hovered: &self.hovered
        };

        let view_transform = view.get_transform(context);
        let graphics_list = view.render(context);
        let children = view.get_children(context);

        let transform = current_transform.multiply(&view_transform);
        let mut hover_z = -1.;
        let mut result = None;

        for graphics in graphics_list {
            let rect = graphics.get_rect()
                .translate(graphics.offset_x, graphics.offset_y)
                .scale(graphics.scale)
                .strip_to_match_aspect_ratio(graphics.aspect_ratio)
                .transform(&transform)
                .multiply(self.virtual_to_real_ratio);

            let primitive = DrawPrimitive {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                z: graphics.z,
                angle: graphics.angle,
                shape: graphics.shape,
                border_color: graphics.border_color.apply_alpha(graphics.border_alpha),
                background_color: graphics.background_color.apply_alpha(graphics.background_alpha),
                overlay_color: graphics.overlay_color.apply_alpha(graphics.overlay_alpha),
                border_radius: graphics.border_radius.to_fixed(&rect, self.virtual_to_real_ratio),
                border_width: graphics.border_width.to_fixed(&rect, self.virtual_to_real_ratio),
                border_dash_length: graphics.border_dash_length.to_fixed(&rect, self.virtual_to_real_ratio),
                border_gap_length: graphics.border_gap_length.to_fixed(&rect, self.virtual_to_real_ratio),
                image_url: graphics.image_url.and_then(|url| Some(Js::get_string_id(&url))),
                image_width: rect.width * graphics.image_scale,
                image_height: rect.height * graphics.image_scale,
                text: graphics.text.and_then(|text| Some(Js::get_string_id(&text))),
                text_font: graphics.text_font,
                text_size: graphics.text_size.to_fixed(&rect, self.virtual_to_real_ratio),
                text_color: graphics.text_color,
                text_margin: graphics.text_margin.to_fixed(&rect, self.virtual_to_real_ratio),
                text_max_width: graphics.text_max_width.unwrap_or(Size::Zero).to_fixed(&rect, self.virtual_to_real_ratio),
                text_max_height: graphics.text_max_height.unwrap_or(Size::Zero).to_fixed(&rect, self.virtual_to_real_ratio),
                text_background_color: graphics.text_background_color,
                text_border_color: graphics.text_border_color,
                text_horizontal_align: graphics.text_horizontal_align,
                text_vertical_align: graphics.text_vertical_align,
                text_bold: graphics.text_bold,
                text_italic: graphics.text_italic,
            };

            Js::draw(primitive);

            if graphics.detectable && rect.contains_point(self.cursor_x, self.cursor_y) {
                hover_z = graphics.z;
            }
        }

        if hover_z > -1. {
            result = Some((view, hover_z));
        }

        for child in children {
            if let Some((child, z)) = self.render_view(child, transform) {
                if z >= hover_z {
                    hover_z = z;
                    result = Some((child, z));
                }
            }
        }

        result
    }
}