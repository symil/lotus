use crate::{client_api::ClientApi, graphics::graphics::Graphics, serialization::serializable::Serializable};

pub trait Entity<Player> : Serializable {
    fn render(&self, _context: &ClientApi<Player>) -> Vec<Graphics> {
        vec![]
    }

    fn hover(&self, _context: &ClientApi<Player>) -> Vec<Graphics> {
        vec![]
    }

    fn is_clickable(&self, _context: &ClientApi<Player>) -> bool {
        true
    }

    fn on_click(&self, _context: &mut ClientApi<Player>) {

    }

    fn get_children(&self, _context: &ClientApi<Player>) -> &[&Self] {
        &[]
    }
}