#![allow(unused_variables)]
use std::rc::Rc;

use crate::{ClientApi, EventHandling, Graphics, KeyboardEvent, MouseEvent, View, WheelEvent};

pub trait Interaction<U, R, E, D> {
    fn is_active(self: Rc<Self>, client: &ClientApi<U, R, E, D>) -> bool { true }
    fn is_exclusive(self: Rc<Self>, client: &ClientApi<U, R, E, D>) -> bool { false }

    fn is_valid_target(self: Rc<Self>, client: &ClientApi<U, R, E, D>, target: &Rc<dyn View<U, R, E, D>>) -> bool { false }
    fn highlight_target(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, target: &Rc<dyn View<U, R, E, D>>, graphics_list: &mut Vec<Graphics>) { }
    fn highlight_target_on_hover(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, target: &Rc<dyn View<U, R, E, D>>, graphics_list: &mut Vec<Graphics>) { }

    fn on_mouse_event(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, event: &MouseEvent) -> EventHandling { EventHandling::Propagate }
    fn on_wheel_event(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, event: &WheelEvent) -> EventHandling { EventHandling::Propagate }
    fn on_keyboard_event(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, event: &KeyboardEvent) -> EventHandling { EventHandling::Propagate }
    fn on_game_event(self: Rc<Self>, client: &mut ClientApi<U, R, E, D>, event: &E) -> EventHandling { EventHandling::Propagate }
}