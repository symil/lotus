use lotus_common::{client_state::ClientState, graphics::graphics::Graphics, traits::{interaction::{Interaction, InteractionResult}, player::Player, view::View}};

#[derive(Debug)]
pub struct DefaultInteraction;

impl<P : Player, V : View<P>> Interaction<P, V> for DefaultInteraction {
    fn is_valid_target(&self, state: &ClientState<P, V>, target: &V) -> bool {
        target.is_clickable(state)
    }

    fn highlight_target_on_hover(&self, state: &ClientState<P, V>, target: &V, graphics_list: &mut Vec<Graphics>) {
        target.hover(state, graphics_list);
    }

    fn on_click(&self, state: &ClientState<P, V>, target: &V) -> InteractionResult {
        target.on_click(state);
        InteractionResult::Keep
    }
}