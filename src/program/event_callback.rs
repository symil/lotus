use crate::utils::Link;
use super::{FunctionBlueprint, TypeBlueprint, Vasm};

#[derive(Debug, Clone)]
pub struct EventCallback {
    pub declarer: Link<TypeBlueprint>,
    pub event_type: Link<TypeBlueprint>,
    pub index_vasm: Vasm,
    pub start: Link<FunctionBlueprint>,
    pub progress: Option<Link<FunctionBlueprint>>,
    pub end: Option<Link<FunctionBlueprint>>,
}