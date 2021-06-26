use std::hash::Hash;

use crate::{view_context::ViewContext, graphics::graphics::Graphics};

pub trait View<P> : Sized + Hash {
    fn root() -> Self;

    fn render(&self, _context: &ViewContext<P, Self>) -> Vec<Graphics> {
        vec![]
    }

    fn hover(&self, _context: &ViewContext<P, Self>) -> Option<Graphics> {
        None
    }

    fn is_clickable(&self, _context: &ViewContext<P, Self>) -> bool {
        true
    }

    fn on_click(&self, _context: &mut ViewContext<P, Self>) {

    }

    fn get_children(&self, _context: &ViewContext<P, Self>) -> Vec<Self> {
        vec![]
    }
}