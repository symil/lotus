#![allow(unused_variables)]

use std::rc::Rc;

use crate::{client_state::ClientState, events::{event_handling::EventHandling, keyboard_event::KeyboardEvent}, graphics::{graphics::Graphics, rect::Rect, transform::Transform}};

pub trait View<P, R, D> {
    fn render(&self, client: &mut ClientState<P, R, D>, rect: &Rect, output: &mut RenderOutput<P, R, D>);
    fn is_clickable(&self, client: &ClientState<P, R, D>) -> bool { true }
    fn hover(&self, client: &ClientState<P, R, D>, graphics_list: &mut Vec<Graphics>) { }
    fn on_click(&self, client: &mut ClientState<P, R, D>) { }
    fn on_keyboard_event(&self, client: &mut ClientState<P, R, D>, event: &KeyboardEvent) -> EventHandling { EventHandling::Propagate }
}

pub struct RenderOutput<P, R, D> {
    pub parent_rect: Rect,
    pub children: Vec<(Rc<dyn View<P, R, D>>, Rect)>,
    pub graphics_list: Vec<Graphics>,
    pub transform: Transform
}

impl<P, R, D> std::fmt::Debug for dyn View<P, R, D>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[View]"))
    }
}

impl<P, R, D> RenderOutput<P, R, D> {
    pub fn new(parent_rect: Rect) -> Self {
        Self {
            parent_rect,
            children: vec![],
            graphics_list: vec![],
            transform: Transform::identity()
        }
    }

    pub fn add_graphics(&mut self, graphics: Graphics) {
        self.graphics_list.push(graphics);
    }

    pub fn add_child<V : View<P, R, D> + 'static>(&mut self, view: V) {
        self.children.push((Rc::new(view), self.parent_rect.clone()));
    }

    pub fn add_child_with_rect<V : View<P, R, D> + 'static>(&mut self, view: V, rect: Rect) {
        self.children.push((Rc::new(view), rect));
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}