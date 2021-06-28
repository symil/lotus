#![allow(unused_variables)]
use std::fmt::Debug;
use crate::{graphics::graphics::Graphics, client_state::ClientState};
use super::{player::Player, view::View};

pub enum InteractionResult {
    Keep,
    Remove
}

pub trait Interaction<P : Player, V : View<P>> : Debug {
    fn is_active(&self, state: &ClientState<P, V>) -> bool { true }
    fn does_grab(&self, state: &ClientState<P, V>) -> bool { false }
    fn is_valid_target(&self, state: &ClientState<P, V>, target: &V) -> bool { false }
    fn highlight_target(&self, state: &ClientState<P, V>, target: &V, graphics_list: &mut Vec<Graphics>) { }
    fn highlight_target_on_hover(&self, state: &ClientState<P, V>, target: &V, graphics_list: &mut Vec<Graphics>) { }
    fn on_click(&self, state: &ClientState<P, V>, target: &V) -> InteractionResult { InteractionResult::Remove }
}