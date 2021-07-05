use std::rc::Rc;

use crate::traits::view::{RenderOutput, View};

use super::rect::Rect;

pub struct SimpleLayout<P, R, D> {
    rect: Rect,
    target: Rect,
    outer_margin: f64,
    inner_margin: f64,
    dx: f64,
    dy: f64,
    fixed_items: Vec<(Rc<dyn View<P, R, D>>, Rect)>,
    movable_items: Vec<(Rc<dyn View<P, R, D>>, Rect)>,
}

impl<P, R, D> SimpleLayout<P, R, D> {
    pub fn new(rect: &Rect) -> Self {
        Self {
            rect: rect.clone(),
            target: rect.scale(0.25),
            outer_margin: 0.,
            inner_margin: 0.,
            dx: 0.,
            dy: 1.,
            fixed_items: vec![],
            movable_items: vec![]
        }
    }

    pub fn scale(mut self, ratio: f64) -> Self {
        self.target = self.target.scale(ratio);
        self
    }

    pub fn pad_to_match_aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.target = self.target.pad_to_match_aspect_ratio(Some(aspect_ratio));
        self
    }

    pub fn strip_to_match_aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.target = self.target.strip_to_match_aspect_ratio(Some(aspect_ratio));
        self
    }

    pub fn outer_margin(mut self, value: f64) -> Self {
        self.outer_margin = value;
        self
    }

    pub fn inner_margin(mut self, value: f64) -> Self {
        self.inner_margin = value;
        self
    }

    pub fn margin(mut self, value: f64) -> Self {
        self.outer_margin = value;
        self.inner_margin = value;
        self
    }

    pub fn move_to(mut self, x: f64, y: f64) -> Self {
        self.target.x = self.rect.width * x;
        self.target.y = self.rect.height * y;
        self
    }

    pub fn width(mut self, width: f64, aspect_ratio: f64) -> Self {
        self.target.width = self.rect.width * width;
        self.target.height = self.target.width / aspect_ratio;
        self
    }

    pub fn height(mut self, height: f64, aspect_ratio: f64) -> Self {
        self.target.height = self.rect.height * height;
        self.target.width = self.target.height * aspect_ratio;
        self
    }

    pub fn push<V : View<P, R, D>>(mut self, view: V) -> Self {
        self
    }

    pub fn towards(mut self, dx: f64, dy: f64) -> Self {
        let mut items_to_fix = self.movable_items.drain(..self.movable_items.len() - 1).collect();
        
        self.fixed_items.append(&mut items_to_fix);

        self.dx = dx;
        self.dy = dy;
        self
    }

    pub fn load(mut self, output: &mut RenderOutput<P, R, D>) {
        output.children.append(&mut self.fixed_items);
        output.children.append(&mut self.movable_items);
    }

    pub fn towards_top(self) -> Self    { self.towards(0., -1.) }
    pub fn towards_bottom(self) -> Self { self.towards(0., 1.) }
    pub fn towards_left(self) -> Self   { self.towards(-1., 0.) }
    pub fn towards_right(self) -> Self  { self.towards(1., 0.) }

    pub fn move_top_left(self) -> Self      { self.move_to(0. , 0. ) }
    pub fn move_top(self) -> Self           { self.move_to(0.5, 0. ) }
    pub fn move_top_right(self) -> Self     { self.move_to(1. , 0. ) }
    pub fn move_left(self) -> Self          { self.move_to(0. , 0.5) }
    pub fn move_center(self) -> Self        { self.move_to(0.5, 0.5) }
    pub fn move_right(self) -> Self         { self.move_to(1. , 0.5) }
    pub fn move_bottom_left(self) -> Self   { self.move_to(0. , 1. ) }
    pub fn move_bottom(self) -> Self        { self.move_to(0.5, 1. ) }
    pub fn move_bottom_right(self) -> Self  { self.move_to(1. , 1. ) }
}