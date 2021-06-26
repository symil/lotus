use std::marker::PhantomData;

use crate::traits::{player::Player, view::View};

pub struct ServerApi<P, V> {
    player_uis: Vec<(u128, V)>,
    _p: PhantomData<P>,
}

impl<P : Player, V : View<P>> ServerApi<P, V> {
    pub fn new() -> Self {
        Self {
            player_uis: vec![],
            _p: PhantomData,
        }
    }

    pub fn set_player_ui(&mut self, player: &P, ui: V) {
        self.player_uis.push((player.get_id(), ui));
    }

    pub fn drain_items(&mut self) -> Vec<(u128, V)> {
        self.player_uis.drain(..).collect()
    }
}