use std::rc::Rc;

use crate::traits::view::{RenderOutput, View};

use super::rect::Rect;

pub struct SimpleLayout<P, R, E, D> {
    rect: Rect,
    target: Rect,
    last: Option<Rect>,
    outer_margin: f64,
    inner_margin: f64,
    dx: f64,
    dy: f64,
    dynamic: bool,
    fixed_items: Vec<(Rc<dyn View<P, R, E, D>>, Rect)>,
    movable_items: Vec<(Rc<dyn View<P, R, E, D>>, Rect)>,
}

impl<P, R, E, D> SimpleLayout<P, R, E, D> {
    pub fn new(rect: &Rect) -> Self {
        Self {
            rect: rect.clone(),
            target: rect.scale(0.25),
            last: None,
            outer_margin: 0.,
            inner_margin: 0.,
            dx: 0.,
            dy: 1.,
            fixed_items: vec![],
            movable_items: vec![],
            dynamic: true
        }
    }

    pub fn dynamic(mut self) -> Self {
        self.dynamic = true;
        self
    }

    pub fn fixed(mut self) -> Self {
        self.dynamic = false;
        self
    }

    pub fn scale(mut self, ratio: f64) -> Self {
        self.target = self.target.scale(ratio);
        self
    }

    pub fn scale_x(mut self, ratio: f64) -> Self {
        self.target.width *= ratio;
        self
    }

    pub fn scale_y(mut self, ratio: f64) -> Self {
        self.target.height *= ratio;
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
        self.flush();
        self.last = None;
        self.target.x = self.rect.width * x;
        self.target.y = self.rect.height * y;
        self
    }

    pub fn resize_from_width(mut self, width: f64, aspect_ratio: f64) -> Self {
        self.target.width = self.rect.width * width;
        self.target.height = self.target.width / aspect_ratio;
        self
    }

    pub fn resize_from_height(mut self, height: f64, aspect_ratio: f64) -> Self {
        self.target.height = self.rect.height * height;
        self.target.width = self.target.height * aspect_ratio;
        self
    }

    pub fn push<V : View<P, R, E, D> + 'static>(mut self, view: V) -> Self {
        let (dx, dy) = self.snap_target_against_last_item();

        self.target.x += dx;
        self.target.y += dy;

        if self.dynamic && self.movable_items.len() > 0 {
            for (_, rect) in self.movable_items.iter_mut() {
                rect.x -= dx / 2.;
                rect.y -= dy / 2.;
            }

            self.target.x -= dx / 2.;
            self.target.y -= dy / 2.;
        }

        let (mut dx, mut dy) = self.snap_target_against_edges();

        if self.dynamic {
            let (dx2, dy2) = self.snap_first_against_edges();

            dx += dx2;
            dy += dy2;

            for (_, rect) in self.movable_items.iter_mut() {
                rect.x += dx;
                rect.y += dy;
            }
        }

        self.target.x += dx;
        self.target.y += dy;

        self.movable_items.push((Rc::new(view), self.target.clone()));
        self.last = Some(self.target.clone());

        self
    }

    fn snap_target_against_last_item(&self) -> (f64, f64) {
        match self.last {
            Some(rect) => {
                let dx = rect.width / 2. + self.inner_margin + self.target.width / 2.;
                let dy = rect.height / 2. + self.inner_margin + self.target.height / 2.;

                let mx = dx / self.dx.abs();
                let my = dy / self.dy.abs();
                let m = f64::min(mx, my);

                (self.dx * m, self.dy * m)
            },
            None => (0., 0.)
        }
    }

    fn snap_against_edges(&self, rect: &Rect) -> (f64, f64) {
        let mut dx : f64 = 0.;
        let mut dy : f64 = 0.;

        dx = dx.max(self.outer_margin - rect.x1());
        dx = dx.min(self.rect.width - self.outer_margin - rect.x2());

        dy = dy.max(self.outer_margin - rect.y1());
        dy = dy.min(self.rect.height - self.outer_margin - rect.y2());

        (dx, dy)
    }

    fn snap_target_against_edges(&self) -> (f64, f64) {
        self.snap_against_edges(&self.target)
    }

    fn snap_first_against_edges(&self) -> (f64, f64) {
        match self.movable_items.first() {
            Some((_, rect)) => self.snap_against_edges(rect),
            None => (0., 0.)
        }
    }

    pub fn towards(mut self, dx: f64, dy: f64) -> Self {
        self.flush();
        self.dx = dx;
        self.dy = dy;
        self
    }

    fn flush(&mut self) {
        self.fixed_items.append(&mut self.movable_items);
    }

    pub fn release(mut self) -> Self {
        self.flush();
        self
    }

    pub fn load(mut self, output: &mut RenderOutput<P, R, E, D>) {
        self.flush();
        output.children.append(&mut self.fixed_items);
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