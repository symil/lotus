use std::rc::Rc;

use lotus_common::{client_state::ClientState, graphics::graphics::Graphics, traits::{interaction::{Interaction, InteractionResult}, local_data::LocalData, player::Player, request::Request, view::View}};

#[derive(Debug)]
pub struct DefaultInteraction;

impl<P : Player, R : Request, D : LocalData> Interaction<P, R, D> for DefaultInteraction {
    fn is_valid_target(&self, state: &ClientState<P, R, D>, target: &Rc<dyn View<P, R, D>>) -> bool {
        target.is_clickable(state)
    }

    fn highlight_target_on_hover(&self, state: &ClientState<P, R, D>, target: &Rc<dyn View<P, R, D>>, graphics_list: &mut Vec<Graphics>) {
        target.hover(state, graphics_list);
    }

    fn on_click(&self, state: &mut ClientState<P, R, D>, target: &Rc<dyn View<P, R, D>>) -> InteractionResult {
        target.on_click(state);
        
        InteractionResult::Keep
    }
}