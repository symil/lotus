use std::{marker::PhantomData, mem::{take}, rc::Rc, usize};

use serializable::Serializable;

use crate::{ClientEvent, DefaultInteraction, Js, JsLogger, ServerMessage, Rect, Transform, ClientApi, EventHandling, MouseAction, DeltaMode, DrawPrimitive, Graphics, Cursor, Size, Interaction, TransitionWrapper, RenderOutput, View, ViewState};

pub struct Client<U, R, E, D> {
    initialized: bool,
    state: Option<ClientApi<U, R, E, D>>,
    virtual_width: f64,
    virtual_height: f64,
    virtual_to_real_ratio: f64,
    cursor_x: f64,
    cursor_y: f64,
    interaction_stack: Vec<Rc<dyn Interaction<U, R, E, D>>>,
    transitions: Vec<TransitionWrapper<U, R, E, D>>,
    create_root: fn() -> Rc<dyn View<U, R, E, D>>,
    window_title: &'static str,
    _e: PhantomData<E>
}

pub struct ClientCreateInfo<U, R, E, D> {
    pub vitual_size: (f64, f64),
    pub create_root: fn() -> Rc<dyn View<U, R, E, D>>,
    pub window_title: &'static str
}

impl<U, R, E, D> Client<U, R, E, D>
    where
        U : Serializable + Default + 'static,
        R : Serializable,
        E : Serializable,
        D : Default
{
    pub fn new(create_info: ClientCreateInfo<U, R, E, D>) -> Self {
        let (virtual_width, virtual_height) = create_info.vitual_size;

        Self {
            initialized: false,
            state: Some(ClientApi::new(JsLogger)),
            virtual_width,
            virtual_height,
            virtual_to_real_ratio: 0.,
            cursor_x: 0.,
            cursor_y: 0.,
            interaction_stack: vec![Rc::new(DefaultInteraction)],
            transitions: vec![],
            create_root: create_info.create_root,
            window_title: create_info.window_title,
            _e: PhantomData
        }
    }

    pub fn start(&mut self) {
        Js::set_window_aspect_ratio(self.virtual_width / self.virtual_height);
        Js::set_window_title(self.window_title);
    }

    pub fn update(&mut self) {
        let mut state = take(&mut self.state).unwrap();
        let window_width = Js::get_window_width();

        self.virtual_to_real_ratio = window_width / self.virtual_width;

        let mut events = vec![];

        while let Some(ui_event) = Js::poll_event() {
            events.push(ui_event.into());
        }

        while let Some(bytes) = Js::poll_message() {
            match <ServerMessage<U, E>>::deserialize(&bytes) {
                Some(message) => {
                    self.initialized = true;
                    state.user = message.user;

                    for game_event in message.events {
                        events.push(ClientEvent::Game(game_event));
                    }
                },
                None => {
                    state.log("failed to deserialize message");
                }
            }
        }

        if self.initialized {
            state.hovered = None;
            state.hover_stack.clear();
            state.all_views.clear();

            let interactions = self.get_active_interactions(&state);
            let window_rect = Rect::from_size(self.virtual_width, self.virtual_height);
            let root_view = (self.create_root)();
            let mut views = self.collect_views(&mut state, root_view, window_rect, Transform::identity(), vec![]); // TODO: maybe re-use the same vector every time
            let hover_stack_indexes = self.compute_hover_stack(&views);
            let hovered_index = hover_stack_indexes.first().map_or(usize::MAX, |index| *index);

            self.render_views(&mut state, &mut views, &interactions, hovered_index);

            state.hovered = views.get(hovered_index).and_then(|view_state| Some(Rc::clone(&view_state.view)));
            state.hover_stack = hover_stack_indexes.into_iter().map(|index| views[index].clone()).collect();
            state.all_views = views;

            self.trigger_events(&mut state, &interactions, events);
            self.trigger_transitions(&mut state);
        }

        for request in &mut state.outgoing_requests.drain(..) {
            Js::send_message(&request.serialize());
        }

        self.state = Some(state);
    }

    fn get_active_interactions(&self, state: &ClientApi<U, R, E, D>) -> Vec<Rc<dyn Interaction<U, R, E, D>>> {
        let mut list = vec![];

        for interaction in &self.interaction_stack {
            if !Rc::clone(interaction).is_active(&state) {
                continue;
            }

            if Rc::clone(interaction).is_exclusive(&state) {
                list.clear();
            }

            list.push(Rc::clone(interaction));
        }

        list
    }

    fn trigger_events(&mut self, state: &mut ClientApi<U, R, E, D>, interactions: &Vec<Rc<dyn Interaction<U, R, E, D>>>, events: Vec<ClientEvent<E>>) {
        for event in events {
            match event {
                ClientEvent::Window(_) => Js::clear_renderer_cache(),
                ClientEvent::Mouse(mut mouse_event) => {
                    mouse_event.x /= self.virtual_to_real_ratio;
                    mouse_event.y /= self.virtual_to_real_ratio;

                    self.cursor_x = mouse_event.x;
                    self.cursor_y = mouse_event.y;

                    if mouse_event.action != MouseAction::Move {
                        if let Some(hovered) = &state.hovered {
                            let hovered = Rc::clone(hovered);

                            for interaction in interactions {
                                if Rc::clone(interaction).is_valid_target(state, &hovered) {
                                    if Rc::clone(interaction).on_mouse_event(state, &mouse_event) == EventHandling::Intercept {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                },
                ClientEvent::Wheel(mut wheel_event) => {
                    if wheel_event.delta_mode == DeltaMode::Pixel {
                        wheel_event.delta_x /= self.virtual_to_real_ratio;
                        wheel_event.delta_y /= self.virtual_to_real_ratio;
                        wheel_event.delta_z /= self.virtual_to_real_ratio;
                    }

                    for interaction in interactions {
                        if Rc::clone(interaction).on_wheel_event(state, &wheel_event) == EventHandling::Intercept {
                            break;
                        }
                    }
                },
                ClientEvent::Keyboard(keyboard_event) => {
                    for interaction in interactions {
                        if Rc::clone(interaction).on_keyboard_event(state, &keyboard_event) == EventHandling::Intercept {
                            break;
                        }
                    }
                },
                ClientEvent::Game(game_event) => {
                    for interaction in interactions {
                        if Rc::clone(interaction).on_game_event(state, &game_event) == EventHandling::Intercept {
                            break;
                        }
                    }
                },
            }
        }
    }

    fn trigger_transitions(&mut self, state: &mut ClientApi<U, R, E, D>) {
        let current_time = Js::get_current_time();

        for transition in state.transitions_to_add.drain(..) {
            let id = transition.get_id();

            if id != 0 {
                for wrapper in &mut self.transitions {
                    if wrapper.id == id {
                        wrapper.ended = true;
                    }
                }
            }

            self.transitions.push(TransitionWrapper::new(transition, id));
        }

        for wrapper in &mut self.transitions {
            if wrapper.ended {
                if !wrapper.started {
                    wrapper.transition.on_start(state);
                }
                wrapper.transition.on_end(state);
                continue;
            }

            if !wrapper.started {
                wrapper.started = true;
                wrapper.start_time = current_time;
                wrapper.duration = wrapper.transition.get_duration();
                wrapper.transition.on_start(state);
            }

            let t = ((current_time - wrapper.start_time) / wrapper.duration).min(1.);

            wrapper.transition.on_progress(state, t);

            if t == 1. {
                wrapper.ended = true;
                wrapper.transition.on_end(state);
            }
        }

        self.transitions.retain(|wrapper| !wrapper.ended);
    }

    fn graphics_to_hitbox(&self, graphics: &Graphics, transform: &Transform) -> Rect {
        graphics.get_rect()
            .translate(graphics.offset_x, graphics.offset_y)
            .scale(graphics.scale)
            .strip_to_match_aspect_ratio(graphics.aspect_ratio)
            .transform(transform)
    }

    fn collect_views(&mut self, state: &mut ClientApi<U, R, E, D>, view: Rc<dyn View<U, R, E, D>>, rect: Rect, current_transform: Transform, views: Vec<ViewState<U, R, E, D>>) -> Vec<ViewState<U, R, E, D>> {
        let mut output = RenderOutput::new(rect.clone());
        
        view.render(state, &rect, &mut output);
        
        let mut view_state = ViewState::new(view);
        let transform = current_transform.multiply(&output.transform);

        if let Some(graphics) = output.graphics_list.first() {
            if graphics.detectable {
                let hitbox = self.graphics_to_hitbox(graphics, &transform);
                
                view_state.hitbox = Some(hitbox);
                view_state.hitbox_z = graphics.z;
            }
        }

        view_state.transform = transform.clone();
        view_state.graphics_list = output.graphics_list;

        let mut result = views;

        result.push(view_state);

        for (child_view, child_rect) in output.children {
            result = self.collect_views(state, child_view, child_rect, transform, result);
        }

        result
    }

    fn compute_hover_stack(&mut self, views: &Vec<ViewState<U, R, E, D>>) -> Vec<usize> {
        let mut views_under_cursor : Vec<(usize, f64)> = views.iter().enumerate().filter_map(|(i, view)| {
            if let Some(hitbox) = view.hitbox {
                match hitbox.contains(self.cursor_x, self.cursor_y) {
                    true => Some((i, view.hitbox_z)),
                    false => None
                }
            } else {
                None
            }
        }).collect();

        views_under_cursor.sort_by(|(_, z1), (_, z2)| z1.partial_cmp(z2).unwrap() );

        views_under_cursor.into_iter().rev().map(|(index, _)| index).collect()
    }

    fn render_views(&mut self, state: &mut ClientApi<U, R, E, D>, views: &mut Vec<ViewState<U, R, E, D>>, interactions: &Vec<Rc<dyn Interaction<U, R, E, D>>>, hovered_index: usize) {
        let mut cursor = Cursor::default();

        Js::clear_canvas();

        for (i, item) in views.iter_mut().enumerate() {
            let is_hovered = hovered_index == i;

            for interaction in interactions {
                if Rc::clone(interaction).is_valid_target(state, &item.view) {
                    Rc::clone(interaction).highlight_target(state, &item.view, &mut item.graphics_list);

                    if is_hovered {
                        Rc::clone(interaction).highlight_target_on_hover(state, &item.view, &mut item.graphics_list);

                        if let Some(graphics) = item.graphics_list.first() {
                            cursor = graphics.cursor;
                        }
                    }
                }
            }

            for graphics in take(&mut item.graphics_list) {
                let rect = self
                    .graphics_to_hitbox(&graphics, &item.transform)
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
                    text_cursor_index: graphics.text_cursor_index.map_or(-1., |value| value as f64)
                };

                Js::draw(primitive);
            }
        }

        Js::set_cursor(cursor);
    }
}