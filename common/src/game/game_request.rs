use crate::{traits::request::Request};
use crate::serialization::*;

#[derive(Debug, Clone, Serializable)]
pub enum GameRequest {
    Login(String),
    Logout,
    A,
    B(u8),
    C(u8, u16),
    D(String, String, String),
    E(u32, u32, u32, u32, u32),
    F(String),
    G(Vec<i16>),
    H([u8; 5], String),
    I(Option<u8>),
    J(Result<String, i16>, u8)
}

impl Request for GameRequest {

}