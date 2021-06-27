use wasm_bindgen::prelude::*;
use as_js_string_macro::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct KeyboardEvent {
    pub action: KeyboardAction,
    pub code: KeyCode,
    pub text: Option<char>,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

#[wasm_bindgen(constructor)]
impl KeyboardEvent {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            action: KeyboardAction::None,
            code: KeyCode::None,
            text: None,
            ctrl: false,
            shift: false,
            alt: false
        }
    }
}

#[as_js_string(lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum KeyboardAction {
    None,
    Down,
    Up
}

#[as_js_string]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    None,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Backquote,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Digit0,
    Minus,
    Equal,
    Backspace,
    Tab,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    BracketLeft,
    BracketRight,
    Enter,
    CapsLock,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    Semicolon,
    Quote,
    Backslash,
    ShiftLeft,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Period,
    Slash,
    ShiftRight,
    ControlLeft,
    MetaLeft,
    AltLeft,
    Space,
    AltRight,
    ContextMenu,
    ControlRight,
    ArrowUp,
    ArrowLeft,
    ArrowDown,
    ArrowRight,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    NumLock,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubstract,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad1,
    Numpad2,
    Numpad3,
    NumpadEnter,
    Numpad0,
    NumpadDecimal,
}