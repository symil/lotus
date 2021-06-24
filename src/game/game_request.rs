use serde::{Serialize, Deserialize};
use crate::traits::request::Request;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameRequest {
    Login(String),
    Logout
}

impl Request for GameRequest {
    fn serialize(value: &Self) -> Vec<u8> {
        bincode::serialize(value).unwrap()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        bincode::deserialize(bytes).unwrap()
    }
}