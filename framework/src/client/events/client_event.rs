use crate::{KeyboardEvent, MouseEvent, WheelEvent, WindowEvent, UiEvent};

pub enum ClientEvent<GameEvent> {
    Window(WindowEvent),
    Mouse(MouseEvent),
    Wheel(WheelEvent),
    Keyboard(KeyboardEvent),
    Game(GameEvent)
}

impl<GameEvent> From<UiEvent> for ClientEvent<GameEvent> {
    fn from(ui_event: UiEvent) -> Self {
        if let Some(window) = ui_event.window {
            Self::Window(window)
        } else if let Some(mouse) = ui_event.mouse {
            Self::Mouse(mouse)
        } else if let Some(wheel) = ui_event.wheel {
            Self::Wheel(wheel)
        } else if let Some(keyboard) = ui_event.keyboard {
            Self::Keyboard(keyboard)
        } else {
            unreachable!()
        }
    }
}