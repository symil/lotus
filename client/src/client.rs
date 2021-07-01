use std::{marker::PhantomData, mem::take, rc::Rc};

use lotus_common::{client_state::ClientState, events::mouse_event::{MouseAction, MouseEvent}, graphics::{graphics::{Cursor, Graphics}, rect::Rect, size::Size, transform::Transform}, traits::{interaction::Interaction, local_data::LocalData, player::Player, request::Request, view::{RectView, View}}};

use crate::{default_interaction::DefaultInteraction, draw_primitive::DrawPrimitive, js::Js};

pub struct Client<P : Player, R : Request, D : LocalData, V : RectView + View<P, R, D>> {
    initialized: bool,
    state: Option<ClientState<P, R, D>>,
    virtual_width: f32,
    virtual_height: f32,
    virtual_to_real_ratio: f32,
    cursor_x: f32,
    cursor_y: f32,
    interaction_stack: Vec<Box<dyn Interaction<P, R, D>>>,
    _v: PhantomData<V>
}

impl<P : Player, R : Request, D : LocalData, V : RectView + View<P, R, D> + 'static> Client<P, R, D, V> {
    pub fn new(virtual_width: f32, virtual_height: f32) -> Self {
        Self {
            initialized: false,
            state: Some(ClientState::new(|string| Js::log(string))),
            virtual_width,
            virtual_height,
            virtual_to_real_ratio: 0.,
            cursor_x: 0.,
            cursor_y: 0.,
            interaction_stack: vec![Box::new(DefaultInteraction)],
            _v: PhantomData
        }
    }

    pub fn start(&mut self) {
        Js::set_window_aspect_ratio(self.virtual_width / self.virtual_height);
    }

    pub fn update(&mut self) {
        let mut state = take(&mut self.state).unwrap();
        let (window_width, _window_height) = Js::get_window_size();

        self.virtual_to_real_ratio = window_width / self.virtual_width;

        while let Some(player) = Js::poll_message::<P>() {
            self.initialized = true;
            state.user = player;
        }

        while let Some(event) = Js::poll_event() {
            if let Some(_) = event.window {
                Js::clear_renderer_cache();
            } else if let Some(mouse_event) = event.mouse {
                self.cursor_x = mouse_event.x;
                self.cursor_y = mouse_event.y;
                self.on_mouse_input(&mut state, mouse_event);
            } else if let Some(_keyboard_event) = event.keyboard {

            }
        }

        for request in &mut state.outgoing_requests.drain(..) {
            Js::send_message(&request);
        }

        if self.initialized {
            let rect = Rect::from_size(self.virtual_width, self.virtual_height);
            let root = Rc::new(V::new(rect));
            let mut views = vec![];
            
            self.collect_views(&mut state, root, Transform::identity(), &mut views);
            state.hovered = self.render_views(&mut state, views);
        }

        self.state = Some(state);
    }

    fn on_mouse_input(&mut self, state: &mut ClientState<P, R, D>, event: MouseEvent) {
        let interactions = self.get_active_interactions(state);

        match event.action {
            MouseAction::Down => {
                if let Some(hovered) = state.hovered.clone() {
                    for interaction in interactions {
                        if interaction.is_valid_target(state, &hovered) {
                            interaction.on_click(state, &hovered);
                        }
                    }
                }
            },
            _ => {}
        }
    }

    fn get_active_interactions(&self, state: &ClientState<P, R, D>) -> Vec<&Box<dyn Interaction<P, R, D>>> {
        let mut list = vec![];

        for interaction in &self.interaction_stack {
            if !interaction.is_active(&state) {
                continue;
            }

            if interaction.does_grab(&state) {
                list.clear();
            }

            list.push(interaction);
        }

        list
    }

    fn collect_views(&mut self, state: &ClientState<P, R, D>, view: Rc<dyn View<P, R, D>>, current_transform: Transform, list: &mut Vec<(Rc<dyn View<P, R, D>>, f32, Vec<Graphics>, Vec<Rect>)>) {
        let view_transform = view.get_transform(state);
        let graphics_list = view.render(state);
        let children = view.get_children(state);
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
            self.collect_views(state, child, transform, list);
        }
    }

    fn render_views(&mut self, state: &ClientState<P, R, D>, list: Vec<(Rc<dyn View<P, R, D>>, f32, Vec<Graphics>, Vec<Rect>)>) -> Option<Rc<dyn View<P, R, D>>> {
        let mut current_z = -1.;
        let mut hovered_index = usize::MAX;
        let mut cursor = Cursor::default();
        let mut result = None;
        let interactions = self.get_active_interactions(state);

        Js::clear_canvas();

        for (i, item) in list.iter().enumerate() {
            let (_, hover_z, _, _) = item;
            
            if *hover_z > -1. && *hover_z >= current_z {
                current_z = *hover_z;
                hovered_index = i;
            }
        }

        for (i, item) in list.into_iter().enumerate() {
            let (view, _, mut graphics_list, rect_list) = item;
            let is_hovered = hovered_index == i;

            for interaction in &interactions {
                if interaction.is_valid_target(state, &view) {
                    interaction.highlight_target(state, &view, &mut graphics_list);

                    if is_hovered {
                        interaction.highlight_target_on_hover(state, &view, &mut graphics_list);
                        cursor = graphics_list[0].cursor;
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

        result
    }
}