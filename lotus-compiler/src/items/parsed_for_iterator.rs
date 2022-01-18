use std::borrow::Cow;
use parsable::{parsable, ItemLocation};
use crate::program::ProgramContext;
use super::{ParsedVarDeclarationNames, ParsedOpeningSquareBracket, Identifier, ParsedCommaToken, ParsedClosingSquareBracket, unwrap_item};

#[parsable]
pub enum ParsedForIterator {
    Item(ParsedVarDeclarationNames),
    IndexAndItem(ParsedIndexAndItem)
}

#[parsable(cascade=true)]
pub struct ParsedIndexAndItem {
    pub opening_bracket: ParsedOpeningSquareBracket,
    pub index_name: Option<Identifier>,
    pub comma: Option<ParsedCommaToken>,
    pub item_names: Option<ParsedVarDeclarationNames>,
    #[parsable(cascade=false)]
    pub closing_bracket: Option<ParsedClosingSquareBracket>
}

impl ParsedForIterator {
    pub fn location(&self) -> &ItemLocation {
        match self {
            ParsedForIterator::Item(item) => item,
            ParsedForIterator::IndexAndItem(item) => item,
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<(Cow<Identifier>, &ParsedVarDeclarationNames)> {
        match self {
            ParsedForIterator::Item(item_names) => Some((
                Cow::Owned(Identifier::unique("index")),
                item_names
            )),
            ParsedForIterator::IndexAndItem(details) => {
                let index_name = unwrap_item(&details.index_name, &details.opening_bracket, context)?;
                let comma = unwrap_item(&details.comma, index_name, context)?;
                let item_names = unwrap_item(&details.item_names, comma, context)?;
                let closing_bracket = unwrap_item(&details.closing_bracket, details, context)?;

                Some((
                    Cow::Borrowed(index_name),
                    item_names
                ))
            },
        }
    }
}