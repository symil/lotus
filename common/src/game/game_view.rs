use crate::client_api::ClientApi;
use crate::graphics::color::Color;
use crate::graphics::rect::Rect;
use crate::graphics::size::Size;
use crate::{client_state::ClientState, graphics::graphics::Graphics, traits::view::View};
use crate::serialization::*;
use super::game_player::GamePlayer;
use super::game_request::GameRequest;
use crate::graphics;

#[derive(Debug, Serializable)]
pub struct GameView {
    rect: Rect
}

impl View<GamePlayer, GameRequest> for GameView {
    fn root(rect: Rect) -> Self {
        GameView { rect }
    }

    fn none() -> Self {
        Self { rect: Rect::default() }
    }

    fn is_none(&self) -> bool {
        self.rect.width < 0.001
    }

    fn hover(&self, _state: &ClientState<GamePlayer, GameRequest, Self>, graphics_list: &mut Vec<Graphics>) {
        graphics_list[0].overlay_color = Color::black().apply_alpha(0.3);
    }

    fn render(&self, _state: &ClientState<GamePlayer, GameRequest, GameView>) -> Vec<Graphics> {
        vec![
            graphics! {
                rect: self.rect,
                scale: 0.5,
                border_radius: Size::Virtual(20.),
                background_color: Color::orange()
            }
        ]
    }

    fn on_click(&self, state: &ClientState<GamePlayer, GameRequest, Self>, api: &mut ClientApi<GameRequest>) {
        state.logger.log("sending \"Hello\"");
        api.send_request(GameRequest::Text("Hello".to_string()));
    }
}