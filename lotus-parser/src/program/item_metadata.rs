use crate::items::VisibilityToken;
use super::Id;

#[derive(Debug)]
pub struct ItemMetadata {
    pub id: Id,
    pub visibility: VisibilityToken,
    pub package_name: String,
    pub file_name: String,
}