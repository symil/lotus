use parsable::{parsable, DataLocation};
use super::{Expression, OpeningRoundBracket, ClosingRoundBracket, CommaToken};

#[parsable]
pub struct ArgumentList {
    opening_bracket: OpeningRoundBracket,
    content: Option<ArgumentListContent>,
    closing_bracket: ClosingRoundBracket,
}

#[parsable]
struct ArgumentListContent {
    first_argument: Expression,
    other_arguments: Vec<OtherArgument>
}

#[parsable]
struct OtherArgument {
    comma: CommaToken,
    expression: Option<Expression>
}

pub struct ArgumentListIterator<'a> {
    index: usize,
    argument_list: &'a ArgumentList
}

impl<'a> Iterator for ArgumentListIterator<'a> {
    type Item = Option<&'a Expression>;

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

impl<'a> IntoIterator for &'a ArgumentList {
    type Item = Option<&'a Expression>;
    type IntoIter = ArgumentListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ArgumentListIterator {
            index: 0,
            argument_list: self,
        }
    }
}

impl ArgumentList {
    pub fn len(&self) -> usize {
        match &self.content {
            Some(content) => 1 + content.other_arguments.len(),
            None => 0,
        }
    }

    pub fn get_location_including_separator(&self, index: usize) -> Option<&DataLocation> {
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

    pub fn get_location(&self, index: usize) -> Option<&DataLocation> {
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