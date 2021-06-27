#![allow(unused_unsafe, unused_imports)]
pub mod js;
pub mod draw_primitive;

use draw_primitive::DrawPrimitive;
use js::Js;
use lotus_common::{events::Event, game::{game_view::GameView, game_player::GamePlayer, game_request::GameRequest}, graphics::{color::Color, graphics::{Cursor, Font, Shape, TextHorizontalAlign, TextVerticalAlign}, rect::Rect}, serialization::serializable::Serializable, traits::view::View, view_context::ViewContext};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    // send(&GameRequest::Login(String::from("Adius")));
}

// static mut DISPLAYED : bool = false;

#[wasm_bindgen]
pub fn update() {
    let image_url_id = Js::get_string_id("img/rondoudou.png");
    let text_id = Js::get_string_id("Some long text\nthat spans over multiple\nlines.");
    let (width, height) = Js::get_window_size();
    let rect = Rect::from_size(width, height);
    let mut primitive = DrawPrimitive {
        background_color: Color::white(),
        border_color: Color::lightgreen(),
        border_width: 20.,
        shape: Shape::Rectangle,
        overlay_color: Color::black().apply_alpha(0.),
        border_radius: 50.,
        line_dash_length: 100.,
        line_gap_length: 50.,
        // image_url: Some(image_url_id),
        image_width: 400.,
        image_height: 400.,
        text: Some(text_id),
        text_size: 40.,
        text_font: Font::Arial,
        text_color: Color::orange(),
        text_margin: 5.,
        text_max_width: 200.,
        text_background_color: Color::grey(),
        text_border_color: Color::black(),
        // text_horizontal_align: TextHorizontalAlign::Left,
        // text_vertical_align: TextVerticalAlign::Top,
        ..DrawPrimitive::default()
    };

    primitive.set_rect(&rect);

    Js::clear_canvas();
    Js::draw(primitive);

    // unsafe {
    //     while let Some(_player) = Js::poll_message::<GamePlayer>() {
    //         if !DISPLAYED {
    //             DISPLAYED = true;

    //             let (width, height) = Js::get_window_size();
    //             let rect = Rect::from_size(width, height);
    //             let mut primitive = DrawPrimitive::from_rect(&rect);

    //             primitive.background_color = Color::rgba(255, 255, 255, 255);

    //             Js::clear_canvas();
    //             Js::draw(primitive);

    //             // let context = ViewContext {
    //             //     rect: Rect::default(),
    //             //     pov: &player,
    //             //     hovered: None
    //             // };

    //             // let ui = GameView::root();
    //             // let graphics = ui.render(&context);
    //         }
    //     }
    // }

    // while let Some(event) = Js::poll_event() {
    //     if let Some(_) = event.window {
    //         Js::log(&Js::get_window_size());
    //     }
    // }
}