use crate::{traits::view::View};

pub struct ViewContext<'a, 'b, P, V : View<P>> {
    pub player: &'a P,
    pub hovered: &'b Option<V>
}