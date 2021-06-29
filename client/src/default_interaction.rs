use lotus_common::{client_api::ClientApi, client_state::ClientState, graphics::graphics::Graphics, traits::{interaction::{Interaction, InteractionResult}, player::Player, request::Request, view::View}};

#[derive(Debug)]
pub struct DefaultInteraction;

impl<P : Player, R : Request> Interaction<P, R> for DefaultInteraction {
    fn is_valid_target(&self, state: &ClientState<P, R>, target: &Box<dyn View<P, R>>) -> bool {
        target.is_clickable(state)
    }

    fn highlight_target_on_hover(&self, state: &ClientState<P, R>, target: &Box<dyn View<P, R>>, graphics_list: &mut Vec<Graphics>) {
        target.hover(state, graphics_list);
    }

    fn on_click(&self, state: &ClientState<P, R>, target: &Box<dyn View<P, R>>, api: &mut ClientApi<R>) -> InteractionResult {
        target.on_click(state, api);
        
        InteractionResult::Keep
    }
}