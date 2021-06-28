#![allow(unused_variables)]
use crate::{graphics::{graphics::Graphics, rect::Rect, transform::Transform}, client_state::ClientState};
use super::{interaction::Interaction, player::Player};

pub trait View<P : Player> : Sized {
    fn root(rect: Rect) -> Self;
    fn none() -> Self;
    fn is_none(&self) -> bool;

    fn render(&self, state: &ClientState<P, Self>) -> Vec<Graphics> { vec![] }
    fn hover(&self, state: &ClientState<P, Self>, graphics_list: &mut Vec<Graphics>) { }
    fn is_clickable(&self, state: &ClientState<P, Self>) -> bool { true }
    fn on_click(&self, state: &ClientState<P, Self>) -> Option<Box<dyn Interaction<P, Self>>> { None }
    fn get_children(&self, state: &ClientState<P, Self>) -> Vec<Self> { vec![] }
    fn get_transform(&self, state: &ClientState<P, Self>) -> Transform { Transform::identity() }
}