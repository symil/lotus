#![allow(unused_variables)]

use std::rc::Rc;

use crate::{client_state::ClientState, graphics::{graphics::Graphics, rect::Rect, transform::Transform}};
use super::{local_data::LocalData, player::Player, request::Request};

pub trait RectView {
    fn new(rect: Rect) -> Self;
}

pub trait View<P : Player, R : Request, D : LocalData> {
    fn render(&self, client: &ClientState<P, R, D>) -> Vec<Graphics> { vec![] }
    fn hover(&self, client: &ClientState<P, R, D>, graphics_list: &mut Vec<Graphics>) { }
    fn is_clickable(&self, client: &ClientState<P, R, D>) -> bool { true }
    fn on_click(&self, client: &mut ClientState<P, R, D>) { }
    fn get_children(&self, client: &ClientState<P, R, D>) -> Vec<Rc<dyn View<P, R, D>>> { vec![] }
    fn get_transform(&self, client: &ClientState<P, R, D>) -> Transform { Transform::identity() }
}

#[macro_export]
macro_rules! make_view {
    ($name:ident) => {
        pub struct $name {
            pub rect: lotus::Rect,
        }

        impl RectView for $name {
            fn new(rect: lotus::Rect) -> Self {
                Self { rect }
            }
        }
    }
}

pub use make_view;

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

//             fn render(&self, client: &lotus::ClientState<$player, $request, Self>) -> Vec<lotus::Graphics> {
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