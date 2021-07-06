use std::{cell::RefCell, rc::Rc};
use lotus_serializable::Serializable;

#[derive(Serializable)]
pub struct ServerMessage<P : Serializable + 'static, E : Serializable> {
    pub player: Rc<RefCell<P>>,
    pub events: Vec<E>
}