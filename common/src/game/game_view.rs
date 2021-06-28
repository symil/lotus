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

    fn hover(&self, graphics_list: &mut Vec<Graphics>, _context: &ViewContext<GamePlayer, Self>) {
        graphics_list[0].overlay_color = Color::black().apply_alpha(0.3);
    }

    fn render(&self, _context: &ViewContext<GamePlayer, GameView>) -> Vec<Graphics> {
        vec![
            graphics! {
                rect: self.rect,
                scale: 0.5,
                border_radius: Size::Virtual(20.),
                background_color: Color::orange()
            }
        ]
    }
}