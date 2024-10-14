use parsable::parsable;

use crate::program::EventCallbackQualifier;

#[parsable]
#[derive(Clone)]
pub struct ParsedEventCallbackQualifierKeyword {
    pub token: ParsedEventCallbackQualifierKeywordToken
}

#[parsable]
#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum ParsedEventCallbackQualifierKeywordToken {
    Standard = "@",
    // TargetSelf = "$",
}

impl ParsedEventCallbackQualifierKeyword {
    pub fn process(&self) -> EventCallbackQualifier {
        match &self.token {
            ParsedEventCallbackQualifierKeywordToken::Standard => EventCallbackQualifier::Standard,
            // ParsedEventCallbackQualifierKeywordToken::TargetSelf => EventCallbackQualifier::TargetSelf,
        }
    }
}