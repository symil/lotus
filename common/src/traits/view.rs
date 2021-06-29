#![allow(unused_variables)]

use crate::{client_api::ClientApi, client_state::ClientState, graphics::{graphics::Graphics, rect::Rect, transform::Transform}};
use super::{player::Player, request::Request};

pub trait RootView {
    fn new(rect: Rect) -> Self;
}

pub trait View<P : Player, R : Request> {
    fn render(&self, state: &ClientState<P, R>) -> Vec<Graphics> { vec![] }
    fn hover(&self, state: &ClientState<P, R>, graphics_list: &mut Vec<Graphics>) { }
    fn is_clickable(&self, state: &ClientState<P, R>) -> bool { true }
    fn on_click(&self, state: &ClientState<P, R>, api: &mut ClientApi<R>) { }
    fn get_children(&self, state: &ClientState<P, R>) -> Vec<Box<dyn View<P, R>>> { vec![] }
    fn get_transform(&self, state: &ClientState<P, R>) -> Transform { Transform::identity() }
}

// #[macro_export]
// macro_rules! make_view {
//     ( $type_name:ident <$player:ident, $request:ident> : $root_type:ident $(, $sub_type:ident)* ) => {
//         pub enum $type_name {
//             None,
//             $root_type($root_type),
//             $( $sub_type($sub_type), )*
//         }

//         $(
//             impl lotus::View<$player, $request> for $sub_type { }
//         )*

//         impl lotus::View<$player, $request> for $type_name {
//             fn root(rect: lotus::Rect) -> Self {
//                 Self::$root_type($root_type::new(rect))
//             }

//             fn none() -> Self {
//                 Self::None
//             }

//             fn is_none(&self) -> bool {
//                 match self {
//                     Self::None => true,
//                     _ => false
//                 }
//             }

//             fn render(&self, state: &lotus::ClientState<$player, $request, Self>) -> Vec<lotus::Graphics> {
//                 match self {
//                     Self::None => vec![],
//                     Self::$root_type(view) => view.render(state),
//                     $( Self::$sub_type(view) => view.render(state), )*
//                 }
//             }
//         }
//     }
// }

// pub use make_view;