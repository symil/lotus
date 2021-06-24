use crate::traits::player::Player;

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
}