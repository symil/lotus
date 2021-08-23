#![allow(unused_variables)]
use std::{rc::Rc};

use crate::{ClientApi, EventHandling, Graphics, Interaction, KeyboardEvent, MouseEvent, View, WheelEvent};

#[derive(Debug)]
pub struct DefaultInteraction;

impl<U, R, E, D> Interaction<U, R, E, D> for DefaultInteraction
    where
        U : Default,
        D : Default
{
    fn is_active(self: Rc<Self>, client: &ClientApi<U, R, E, D>) -> bool {
        true
    }

    fn is_valid_target(self: Rc<Self>, client: &ClientApi<U, R, E, D>, target: &Rc<dyn View<U, R, E, D>>) -> bool {
        true
    }

    fn highlight_target(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, target: &Rc<dyn View<U, R, E, D>>, graphics_list: &mut Vec<Graphics>) {

    }

    fn highlight_target_on_hover(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, target: &Rc<dyn View<U, R, E, D>>, graphics_list: &mut Vec<Graphics>) {
        target.hover(client, graphics_list);
    }

    fn on_mouse_event(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, event: &MouseEvent) -> EventHandling {
        let views = client.take_views();

        for view_state in &views.hover_stack {
            if view_state.view.on_mouse_event(client, event, &view_state.transform) == EventHandling::Intercept {
                break;
            }
        }

        client.set_views(views);

        EventHandling::Propagate
    }

    fn on_wheel_event(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, event: &WheelEvent) -> EventHandling {
        let views = client.take_views();

        for view_state in &views.hover_stack {
            if view_state.view.on_wheel_event(client, event) == EventHandling::Intercept {
                break;
            }
        }

        client.set_views(views);

        EventHandling::Propagate
    }

    fn on_keyboard_event(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, event: &KeyboardEvent) -> EventHandling {
        let views = client.take_views();

        for view_state in &views.all {
            if view_state.view.on_keyboard_event(client, event) == EventHandling::Intercept {
                break;
            }
        }

        client.set_views(views);

        EventHandling::Propagate
    }

    fn on_game_event(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, event: &E) -> EventHandling {
        let views = client.take_views();

        for view_state in &views.all {
            if view_state.view.on_game_event(client, event) == EventHandling::Intercept {
                break;
            }
        }

        client.set_views(views);

        EventHandling::Propagate
    }
}