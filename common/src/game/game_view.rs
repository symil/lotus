use crate::graphics::color::Color;
use crate::graphics::rect::Rect;
use crate::graphics::size::Size;
use crate::{view_context::ViewContext, graphics::graphics::Graphics, traits::view::View};
use crate::serialization::*;
use super::game_player::GamePlayer;
use crate::graphics;

#[derive(Debug, Serializable)]
pub struct GameView {
    rect: Rect
}

impl View<GamePlayer> for GameView {
    fn root(rect: Rect) -> Self {
        GameView { rect }
    }

    fn render(&self, _context: &ViewContext<GamePlayer, GameView>) -> Vec<Graphics> {
        vec![
            graphics! {
                rect: self.rect,
                background_color: Color::white()
            },
            graphics! {
                rect: self.rect,
                scale: 0.5,
                border_radius: Size::Virtual(20.),
                background_color: Color::orange()
            }
        ]
    }
}