#![allow(unused_variables)]
use std::rc::Rc;

use crate::{client_state::ClientState, graphics::graphics::Graphics};
use super::{view::View};

pub enum InteractionResult {
    Keep,
    Remove
}

pub trait Interaction<P, R, D> {
    fn is_active(&self, client: &ClientState<P, R, D>) -> bool { true }
    fn does_grab(&self, client: &ClientState<P, R, D>) -> bool { false }
    fn is_valid_target(&self, client: &ClientState<P, R, D>, target: &Rc<dyn View<P, R, D>>) -> bool { false }
    fn highlight_target(&self, client: &ClientState<P, R, D>, target: &Rc<dyn View<P, R, D>>, graphics_list: &mut Vec<Graphics>) { }
    fn highlight_target_on_hover(&self, client: &ClientState<P, R, D>, target: &Rc<dyn View<P, R, D>>, graphics_list: &mut Vec<Graphics>) { }
    fn on_click(&self, client: &mut ClientState<P, R, D>, target: &Rc<dyn View<P, R, D>>) -> InteractionResult { InteractionResult::Remove }
}