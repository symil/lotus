use std::marker::PhantomData;

use crate::traits::{entity::Entity, player::Player};

pub struct ServerApi<P, E> {
    player_uis: Vec<(u128, E)>,
    _p: PhantomData<P>
}

impl<P : Player, E : Entity<P>> ServerApi<P, E> {
    pub fn new() -> Self {
        Self {
            player_uis: vec![],
            _p: PhantomData
        }
    }

    pub fn set_player_ui(&mut self, player: &P, ui: E) {
        self.player_uis.push((player.get_id(), ui));
    }

    pub fn drain_items(&mut self) -> Vec<(u128, E)> {
        self.player_uis.drain(..).collect()
    }
}