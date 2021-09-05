use wasm_bindgen::prelude::*;
use enum_as_string_macro::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct WheelEvent {
    pub delta_x: f64,
    pub delta_y: f64,
    pub delta_z: f64,
    pub delta_mode: DeltaMode
}

#[wasm_bindgen(constructor)]
impl WheelEvent {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            delta_x: 0.,
            delta_y: 0.,
            delta_z: 0.,
            delta_mode: DeltaMode::Line
        }
    }
}

#[enum_as_string(lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeltaMode {
    Pixel,
    Line,
    Page
}