use lotus_common::{client_api::ClientApi, client_state::ClientState, events::mouse_event::{MouseAction, MouseEvent}, graphics::{graphics::{Cursor, Graphics}, rect::Rect, size::Size, transform::Transform}, traits::{interaction::Interaction, player::Player, request::Request, view::View}};

use crate::{default_interaction::DefaultInteraction, draw_primitive::DrawPrimitive, js::Js};

#[derive(Debug)]
pub struct Client<P : Player, R : Request, V : View<P, R>> {
    initialized: bool,
    state: ClientState<P, R, V>,
    virtual_width: f32,
    virtual_height: f32,
    virtual_to_real_ratio: f32,
    cursor_x: f32,
    cursor_y: f32,
    interaction_stack: Vec<Box<dyn Interaction<P, R, V>>>
}

impl<P : Player, R : Request, V : View<P, R>> Client<P, R, V> {
    pub fn new(virtual_width: f32, virtual_height: f32) -> Self {
        Self {
            initialized: false,
            state: ClientState::new(|string| Js::log(string)),
            virtual_width,
            virtual_height,
            virtual_to_real_ratio: 0.,
            cursor_x: 0.,
            cursor_y: 0.,
            interaction_stack: vec![Box::new(DefaultInteraction)]
        }
    }

    pub fn start(&mut self) {
        Js::set_window_aspect_ratio(self.virtual_width / self.virtual_height);
    }

    pub fn update(&mut self) {
        let (real_width, _real_height) = Js::get_window_size();

        self.virtual_to_real_ratio = real_width / self.virtual_width;

        while let Some(player) = Js::poll_message::<P>() {
            self.initialized = true;
            self.state.user = player;
        }

        let mut api = ClientApi::new();

        while let Some(event) = Js::poll_event() {
            if let Some(_) = event.window {
                Js::clear_renderer_cache();
            } else if let Some(mouse_event) = event.mouse {
                self.cursor_x = mouse_event.x;
                self.cursor_y = mouse_event.y;
                self.on_mouse_input(mouse_event, &mut api);
            } else if let Some(_keyboard_event) = event.keyboard {

            }
        }

        for request in api.poll_requests() {
            Js::send_message(&request);
        }

        if self.initialized {
            let rect = Rect::from_size(self.virtual_width, self.virtual_height);
            let root = V::root(rect);
            let mut views = vec![];
            
            self.collect_views(root, Transform::identity(), &mut views);
            self.state.hovered = self.render_views(views);
        }
    }

    fn on_mouse_input(&mut self, event: MouseEvent, api: &mut ClientApi<R>) {
        let interactions = self.get_active_interactions();

        match event.action {
            MouseAction::Down => {
                if !self.state.hovered.is_none() {
                    for interaction in interactions {
                        if interaction.is_valid_target(&self.state, &self.state.hovered) {
                            interaction.on_click(&self.state, &self.state.hovered, api);
                        }
                    }
                }
            },
            _ => {}
        }
    }

    fn get_active_interactions(&self) -> Vec<&Box<dyn Interaction<P, R, V>>> {
        let mut list = vec![];

        for interaction in &self.interaction_stack {
            if !interaction.is_active(&self.state) {
                continue;
            }

            if interaction.does_grab(&self.state) {
                list.clear();
            }

            list.push(interaction);
        }

        list
    }

    fn collect_views(&mut self, view: V, current_transform: Transform, list: &mut Vec<(V, f32, Vec<Graphics>, Vec<Rect>)>) {
        let view_transform = view.get_transform(&self.state);
        let graphics_list = view.render(&self.state);
        let children = view.get_children(&self.state);
        let transform = current_transform.multiply(&view_transform);
        let mut rect_list = vec![];
        let mut hover_z = -1.;

        for graphics in &graphics_list {
            let rect = graphics.get_rect()
                .translate(graphics.offset_x, graphics.offset_y)
                .scale(graphics.scale)
                .strip_to_match_aspect_ratio(graphics.aspect_ratio)
                .transform(&transform)
                .multiply(self.virtual_to_real_ratio);

            if graphics.detectable && graphics.z > hover_z && rect.contains(self.cursor_x, self.cursor_y) {
                hover_z = graphics.z;
            }

            rect_list.push(rect);
        }

        list.push((view, hover_z, graphics_list, rect_list));

        for child in children {
            self.collect_views(child, transform, list);
        }
    }

    fn render_views(&mut self, list: Vec<(V, f32, Vec<Graphics>, Vec<Rect>)>) -> V {
        let mut current_z = -1.;
        let mut hovered_index = usize::MAX;
        let mut cursor = Cursor::default();
        let mut result = None;
        let interactions = self.get_active_interactions();

        Js::clear_canvas();

        for (i, item) in list.iter().enumerate() {
            let (_, hover_z, graphics_list, _) = item;
            
            if *hover_z > -1. && *hover_z >= current_z {
                current_z = *hover_z;
                hovered_index = i;
                cursor = graphics_list[0].cursor;
            }
        }

        for (i, item) in list.into_iter().enumerate() {
            let (view, _, mut graphics_list, rect_list) = item;
            let is_hovered = hovered_index == i;

            for interaction in &interactions {
                if interaction.is_valid_target(&self.state, &self.state.hovered) {
                    interaction.highlight_target(&self.state, &self.state.hovered, &mut graphics_list);

                    if is_hovered {
                        interaction.highlight_target_on_hover(&self.state, &self.state.hovered, &mut graphics_list);
                    }
                }
            }

            if is_hovered {
                result = Some(view);
            }

            for (graphics, rect) in graphics_list.into_iter().zip(rect_list.into_iter()) {
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
            }
        }

        Js::set_cursor(cursor);

        match result {
            None => V::none(),
            Some(view) => view
        }
    }
}