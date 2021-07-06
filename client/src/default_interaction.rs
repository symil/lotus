use std::rc::Rc;

use lotus_common::{client_state::ClientState, graphics::graphics::Graphics, traits::{interaction::{Interaction, InteractionResult}, view::View}};

#[derive(Debug)]
pub struct DefaultInteraction;

impl<P, R, E, D> Interaction<P, R, E, D> for DefaultInteraction {
    fn is_valid_target(&self, state: &ClientState<P, R, E, D>, target: &Rc<dyn View<P, R, E, D>>) -> bool {
        target.is_clickable(state)
    }

    fn highlight_target_on_hover(&self, state: &ClientState<P, R, E, D>, target: &Rc<dyn View<P, R, E, D>>, graphics_list: &mut Vec<Graphics>) {
        target.hover(state, graphics_list);
    }

    fn on_click(&self, state: &mut ClientState<P, R, E, D>, target: &Rc<dyn View<P, R, E, D>>) -> InteractionResult {
        target.on_click(state);
        
        InteractionResult::Keep
    }
}