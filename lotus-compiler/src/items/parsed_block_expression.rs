use parsable::parsable;
use crate::{program::{ProgramContext, ScopeKind, Type, Vasm}};
use super::{ParsedExpression, ParsedSemicolonToken};

#[parsable(name="block")]
pub struct ParsedBlockExpression {
    #[parsable(brackets="{}")]
    pub list: Vec<ParsedBlockItem>
}

#[parsable]
pub struct ParsedBlockItem {
    pub expression: ParsedExpression,
    pub semicolon: Option<ParsedSemicolonToken>
}

impl ParsedBlockExpression {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = context.vasm().set_void(context);

        context.push_scope(ScopeKind::Block);

        for (i, item) in self.list.iter().enumerate() {
            let is_last = i == self.list.len() - 1;
            let hint = match is_last {
                true => type_hint,
                false => None,
            };

            if let Some(item_vasm) = item.expression.process(hint, context) {
                result = result.append(item_vasm);

                if !is_last || item.semicolon.is_some() {
                    let ty = result.ty.clone();

                    result = result
                        .drop(&ty)
                        .set_type(context.void_type());
                }

                // if !is_last && item.semicolon.is_none() {
                //     context.errors.add(&item.location.get_end(), format!("missing `;`"));
                // }
            }
        }

        context.pop_scope();

        Some(result)
    }
}