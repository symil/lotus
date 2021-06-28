#![allow(unused_variables)]
use std::fmt::Debug;
use crate::{client_api::ClientApi, client_state::ClientState, graphics::graphics::Graphics};
use super::{player::Player, request::Request, view::View};

pub enum InteractionResult {
    Keep,
    Remove
}

pub trait Interaction<P : Player, R : Request, V : View<P, R>> : Debug {
    fn is_active(&self, state: &ClientState<P, R, V>) -> bool { true }
    fn does_grab(&self, state: &ClientState<P, R, V>) -> bool { false }
    fn is_valid_target(&self, state: &ClientState<P, R, V>, target: &V) -> bool { false }
    fn highlight_target(&self, state: &ClientState<P, R, V>, target: &V, graphics_list: &mut Vec<Graphics>) { }
    fn highlight_target_on_hover(&self, state: &ClientState<P, R, V>, target: &V, graphics_list: &mut Vec<Graphics>) { }
    fn on_click(&self, state: &ClientState<P, R, V>, target: &V, api: &mut ClientApi<R>) -> InteractionResult { InteractionResult::Remove }
}