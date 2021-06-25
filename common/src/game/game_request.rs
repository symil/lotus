use crate::{traits::request::Request};
use crate::serialization::serializable::Serializable;

#[derive(Debug, Clone, Serializable)]
pub enum GameRequest {
    A,
    B(u8),
    C(u8, u16),
    D(String, String, String),
    E(u32, u32, u32, u32, u32),
    F(String),
    G(Vec<i16>)
}

impl Request for GameRequest {

}