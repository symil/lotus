use lotus_serializable::Serializable;

use crate::traits::world::Id;

#[derive(Serializable)]
pub struct ServerMessage<W : Serializable + 'static, E : Serializable> {
    pub user: Id,
    pub world: W,
    pub events: Vec<E>
}