use crate::serialization::serializable::Serializable;

#[derive(Debug, Serializable)]
pub struct StateMessage<P : Serializable, E : Serializable> {
    pub player: P,
    pub ui: E
}

impl<P : Serializable + Clone, E : Serializable> StateMessage<P, E> {
    pub fn new(player: &P, ui: E) -> Self {
        Self {
            player: player.clone(),
            ui
        }
    }
}