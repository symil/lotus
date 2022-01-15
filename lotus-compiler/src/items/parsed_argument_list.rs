use parsable::{parsable, ItemLocation};
use super::{ParsedExpression, ParsedOpeningRoundBracket, ParsedClosingRoundBracket, ParsedComma};

#[parsable]
pub struct ParsedArgumentList {
    opening_bracket: ParsedOpeningRoundBracket,
    content: Option<ParsedArgumentListContent>,
    closing_bracket: ParsedClosingRoundBracket,
}

#[parsable]
struct ParsedArgumentListContent {
    first_argument: ParsedExpression,
    other_arguments: Vec<ParsedOtherArgument>
}

#[parsable]
struct ParsedOtherArgument {
    comma: ParsedComma,
    expression: Option<ParsedExpression>
}

pub struct ArgumentListIterator<'a> {
    index: usize,
    argument_list: &'a ParsedArgumentList
}

impl<'a> Iterator for ArgumentListIterator<'a> {
    type Item = Option<&'a ParsedExpression>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        if let Some(content) = &self.argument_list.content {
            if self.index == 0 {
                result = Some(Some(&content.first_argument));
            } else {
                result = content.other_arguments.get(self.index - 1).map(|arg| arg.expression.as_ref());
            }
        }

        if result.is_some() {
            self.index += 1;
        }

        result
    }
}

impl<'a> IntoIterator for &'a ParsedArgumentList {
    type Item = Option<&'a ParsedExpression>;
    type IntoIter = ArgumentListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ArgumentListIterator {
            index: 0,
            argument_list: self,
        }
    }
}

impl ParsedArgumentList {
    pub fn len(&self) -> usize {
        match &self.content {
            Some(content) => 1 + content.other_arguments.len(),
            None => 0,
        }
    }

    pub fn get_location_including_separator(&self, index: usize) -> Option<&ItemLocation> {
        let mut result = None;

        if let Some(content) = &self.content {
            if index == 0 {
                result = Some(&content.first_argument.location);
            } else {
                result = content.other_arguments.get(index - 1).map(|arg| &arg.location);
            }
        }

        result
    }

    pub fn get_location(&self, index: usize) -> Option<&ItemLocation> {
        let mut result = None;

        if let Some(content) = &self.content {
            if index == 0 {
                result = Some(&content.first_argument.location);
            } else {
                result = content.other_arguments.get(index - 1).and_then(|arg| arg.expression.as_ref().map(|expr| &expr.location));
            }
        }

        result
    }
}