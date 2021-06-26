use crate::{graphics::rect::Rect, traits::view::View};

pub struct ViewContext<'a, 'b, P, V : View<P>> {
    pub rect: Rect,
    pub pov: &'a P,
    pub hovered: Option<&'b V>
}