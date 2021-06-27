use wasm_bindgen::prelude::*;
use self::{keyboard_event::KeyboardEvent, mouse_event::MouseEvent, window_event::WindowEvent};

pub mod mouse_event;
pub mod keyboard_event;
pub mod window_event;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Event {
    pub window: Option<WindowEvent>,
    pub mouse: Option<MouseEvent>,
    pub keyboard: Option<KeyboardEvent>
}

#[wasm_bindgen(constructor)]
impl Event {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            window: None,
            mouse: None,
            keyboard: None,
        }
    }
}