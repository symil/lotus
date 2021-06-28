use lotus_common::{client_api::ClientApi, client_state::ClientState, graphics::graphics::Graphics, traits::{interaction::{Interaction, InteractionResult}, player::Player, request::Request, view::View}};

#[derive(Debug)]
pub struct DefaultInteraction;

impl<P : Player, R : Request, V : View<P, R>> Interaction<P, R, V> for DefaultInteraction {
    fn is_valid_target(&self, state: &ClientState<P, R, V>, target: &V) -> bool {
        target.is_clickable(state)
    }

    fn highlight_target_on_hover(&self, state: &ClientState<P, R, V>, target: &V, graphics_list: &mut Vec<Graphics>) {
        target.hover(state, graphics_list);
    }

    fn on_click(&self, state: &ClientState<P, R, V>, target: &V, api: &mut ClientApi<R>) -> InteractionResult {
        target.on_click(state, api);
        
        InteractionResult::Keep
    }
}