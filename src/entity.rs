use crate::{client_api::ClientApi, graphics::graphics::Graphics};

pub trait Entity<C> {
    type C;

    fn render(&self, _context: &ClientApi<C>) -> Vec<Graphics> {
        vec![]
    }

    fn hover(&self, _context: &ClientApi<C>) -> Vec<Graphics> {
        vec![]
    }

    fn is_clickable(&self, _context: &ClientApi<C>) -> bool {
        true
    }

    fn on_click(&self, _context: &mut ClientApi<C>) {

    }

    fn get_children(&self, _context: &ClientApi<C>) -> &[&Self] {
        &[]
    }
}