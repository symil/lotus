use crate::traits::player::Player;
use crate::serialization::serializable::Serializable;

#[derive(Debug, Serializable)]
pub struct GamePlayer {
    pub id: u128,
    pub username: String
}

impl Player for GamePlayer {
    fn from_id(id: u128) -> Self {
        let mut str = id.to_string();

        str.truncate(5);

        Self {
            id,
            username: format!("#{}", str)
        }
    }

    fn get_id(&self) -> u128 {
        self.id
    }
}