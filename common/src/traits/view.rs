#![allow(unused_variables)]
use crate::{client_api::ClientApi, client_state::ClientState, graphics::{graphics::Graphics, rect::Rect, transform::Transform}};
use super::{player::Player, request::Request};

pub trait View<P : Player, R : Request> : Sized {
    fn root(rect: Rect) -> Self;
    fn none() -> Self;
    fn is_none(&self) -> bool;

    fn render(&self, state: &ClientState<P, R, Self>) -> Vec<Graphics> { vec![] }
    fn hover(&self, state: &ClientState<P, R, Self>, graphics_list: &mut Vec<Graphics>) { }
    fn is_clickable(&self, state: &ClientState<P, R, Self>) -> bool { true }
    fn on_click(&self, state: &ClientState<P, R, Self>, api: &mut ClientApi<R>);
    fn get_children(&self, state: &ClientState<P, R, Self>) -> Vec<Self> { vec![] }
    fn get_transform(&self, state: &ClientState<P, R, Self>) -> Transform { Transform::identity() }
}