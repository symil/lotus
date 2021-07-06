#![allow(unused_variables)]
use std::{rc::Rc};

use lotus_common::{client_state::ClientState, events::{event_handling::EventHandling, keyboard_event::KeyboardEvent, mouse_event::MouseEvent, wheel_event::WheelEvent}, graphics::graphics::Graphics, traits::{interaction::{Interaction}, view::View}};

#[derive(Debug)]
pub struct DefaultInteraction;

impl<P : Default, R, E, D : Default> Interaction<P, R, E, D> for DefaultInteraction {
    fn is_active(self: Rc<Self>, client: &ClientState<P, R, E, D>) -> bool {
        true
    }

    fn is_valid_target(self: Rc<Self>, client: &ClientState<P, R, E, D>, target: &Rc<dyn View<P, R, E, D>>) -> bool {
        true
    }

    fn highlight_target(self: Rc<Self>, client: &mut ClientState<P, R, E, D>, target: &Rc<dyn View<P, R, E, D>>, graphics_list: &mut Vec<Graphics>) {

    }

    fn highlight_target_on_hover(self: Rc<Self>, client: &mut ClientState<P, R, E, D>, target: &Rc<dyn View<P, R, E, D>>, graphics_list: &mut Vec<Graphics>) {
        target.hover(client, graphics_list);
    }

    fn on_mouse_event(self: Rc<Self>, client: &mut ClientState<P, R, E, D>, event: &MouseEvent) -> EventHandling {
        let views = client.take_views();

        if let Some(view) = &views.hovered {
            view.on_mouse_event(client, event);
        }

        client.set_views(views);

        EventHandling::Intercept
    }

    fn on_wheel_event(self: Rc<Self>, client: &mut ClientState<P, R, E, D>, event: &WheelEvent) -> EventHandling {
        let views = client.take_views();

        for view in &views.hover_stack {
            if view.on_wheel_event(client, event) == EventHandling::Intercept {
                break;
            }
        }

        client.set_views(views);

        EventHandling::Intercept
    }

    fn on_keyboard_event(self: Rc<Self>, client: &mut ClientState<P, R, E, D>, event: &KeyboardEvent) -> EventHandling {
        let views = client.take_views();

        for view in &views.all {
            if view.on_keyboard_event(client, event) == EventHandling::Intercept {
                break;
            }
        }

        client.set_views(views);

        EventHandling::Intercept
    }

    fn on_game_event(self: Rc<Self>, client: &mut ClientState<P, R, E, D>, event: &E) -> EventHandling {
        let views = client.take_views();

        for view in &views.all {
            if view.on_game_event(client, event) == EventHandling::Intercept {
                break;
            }
        }

        client.set_views(views);

        EventHandling::Intercept
    }
}