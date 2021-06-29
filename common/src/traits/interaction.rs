#![allow(unused_variables)]
use crate::{client_api::ClientApi, client_state::ClientState, graphics::graphics::Graphics};
use super::{player::Player, request::Request, view::View};

pub enum InteractionResult {
    Keep,
    Remove
}

pub trait Interaction<P : Player, R : Request> {
    fn is_active(&self, state: &ClientState<P, R>) -> bool { true }
    fn does_grab(&self, state: &ClientState<P, R>) -> bool { false }
    fn is_valid_target(&self, state: &ClientState<P, R>, target: &Box<dyn View<P, R>>) -> bool { false }
    fn highlight_target(&self, state: &ClientState<P, R>, target: &Box<dyn View<P, R>>, graphics_list: &mut Vec<Graphics>) { }
    fn highlight_target_on_hover(&self, state: &ClientState<P, R>, target: &Box<dyn View<P, R>>, graphics_list: &mut Vec<Graphics>) { }
    fn on_click(&self, state: &ClientState<P, R>, target: &Box<dyn View<P, R>>, api: &mut ClientApi<R>) -> InteractionResult { InteractionResult::Remove }
}