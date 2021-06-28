use crate::{graphics::{graphics::Graphics, rect::Rect, transform::Transform}, view_context::ViewContext};

pub trait View<P> : Sized {
    fn root(rect: Rect) -> Self;

    fn render(&self, _context: &ViewContext<P, Self>) -> Vec<Graphics> {
        vec![]
    }

    fn hover(&self, _graphics_list: &mut Vec<Graphics>, _context: &ViewContext<P, Self>) {

    }

    fn is_clickable(&self, _context: &ViewContext<P, Self>) -> bool {
        true
    }

    fn on_click(&self, _context: &mut ViewContext<P, Self>) {

    }

    fn get_children(&self, _context: &ViewContext<P, Self>) -> Vec<Self> {
        vec![]
    }

    fn get_transform(&self, _context: &ViewContext<P, Self>) -> Transform {
        Transform::identity()
    }
}