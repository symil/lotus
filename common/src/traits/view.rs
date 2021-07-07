#![allow(unused_variables)]

use std::rc::Rc;

use crate::{client_state::ClientState, events::{event_handling::EventHandling, keyboard_event::KeyboardEvent, mouse_event::MouseEvent, wheel_event::WheelEvent}, graphics::{graphics::Graphics, rect::Rect, transform::Transform}};

pub trait View<W, R, E, D> {
    fn render(&self, client: &mut ClientState<W, R, E, D>, rect: &Rect, output: &mut RenderOutput<W, R, E, D>);
    fn hover(&self, client: &mut ClientState<W, R, E, D>, graphics_list: &mut Vec<Graphics>) { }

    fn on_mouse_event(&self, client: &mut ClientState<W, R, E, D>, event: &MouseEvent) -> EventHandling { EventHandling::Propagate }
    fn on_wheel_event(&self, client: &mut ClientState<W, R, E, D>, event: &WheelEvent) -> EventHandling { EventHandling::Propagate }
    fn on_keyboard_event(&self, client: &mut ClientState<W, R, E, D>, event: &KeyboardEvent) -> EventHandling { EventHandling::Propagate }
    fn on_game_event(&self, client: &mut ClientState<W, R, E, D>, event: &E) -> EventHandling { EventHandling::Propagate }
}

impl<W, R, E, D> std::fmt::Debug for dyn View<W, R, E, D>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[View]"))
    }
}

pub struct RenderOutput<W, R, E, D> {
    // this field should not be here, as it's an input and not an output
    // leaving it here for convenience for now
    pub parent_rect: Rect,

    pub children: Vec<(Rc<dyn View<W, R, E, D>>, Rect)>,
    pub graphics_list: Vec<Graphics>,
    pub transform: Transform
}

impl<W, R, E, D> RenderOutput<W, R, E, D> {
    pub fn new(rect: Rect) -> Self {
        Self {
            parent_rect: rect,
            children: vec![],
            graphics_list: vec![],
            transform: Transform::identity()
        }
    }

    pub fn add_graphics(&mut self, graphics: Graphics) {
        self.graphics_list.push(graphics);
    }

    pub fn add_child<V : View<W, R, E, D> + 'static>(&mut self, view: V) {
        self.children.push((Rc::new(view), self.parent_rect.clone()));
    }

    pub fn add_child_with_rect<V : View<W, R, E, D> + 'static>(&mut self, view: V, rect: Rect) {
        self.children.push((Rc::new(view), rect));
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

pub struct ViewState<W, R, E, D> {
    pub view: Rc<dyn View<W, R, E, D>>,
    pub hitbox: Option<Rect>,
    pub hitbox_z: f64,
    pub graphics_list: Vec<Graphics>,
    pub transform: Transform
}

impl<W, R, E, D> ViewState<W, R, E, D> {
    pub fn new(view: Rc<dyn View<W, R, E, D>>,) -> Self {
        Self {
            view,
            hitbox: None,
            hitbox_z: 0.,
            graphics_list: vec![],
            transform: Transform::identity()
        }
    }
}