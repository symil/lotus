use crate::{view_context::ViewContext, graphics::graphics::Graphics, traits::view::View};
use crate::serialization::*;
use super::game_player::GamePlayer;

#[derive(Debug, Serializable, Hash)]
pub struct GameView {
}

impl View<GamePlayer> for GameView {
    fn root() -> Self {
        GameView {}
    }

    fn render(&self, _context: &ViewContext<GamePlayer, GameView>) -> Vec<Graphics> {
        vec![Graphics::default()]
    }
}