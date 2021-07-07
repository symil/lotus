use lotus_serializable::Serializable;

#[derive(Serializable)]
pub struct ServerMessage<U : Serializable + 'static, E : Serializable> {
    pub user: U,
    pub events: Vec<E>
}