use crate::{client_api::ClientApi, graphics::graphics::Graphics, traits::entity::Entity};
use crate::serialization::*;
use super::game_player::GamePlayer;

#[derive(Debug, Serializable)]
pub struct GameEntity {
}

impl Entity<GamePlayer> for GameEntity {
    fn render(&self, _context: &ClientApi<GamePlayer>) -> Vec<Graphics> {
        vec![Graphics::default()]
    }
}