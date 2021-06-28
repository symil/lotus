use std::fmt::Debug;

use crate::{logger::Logger, traits::{player::Player, view::View}};

#[derive(Clone, Debug)]
pub struct ClientState<P : Player, V : View<P>> {
    pub logger: Logger,
    pub user: P,
    pub hovered: V
}