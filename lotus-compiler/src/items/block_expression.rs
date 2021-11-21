use parsable::parsable;
use crate::{program::{ProgramContext, Type, VI, Vasm}, vasm};
use super::Expression;

#[parsable]
pub struct BlockExpression {
    #[parsable(brackets="{}")]
    pub list: Vec<BlockItem>
}

#[parsable]
pub struct BlockItem {
    pub expression: Expression,
    #[parsable(value=";")]
    pub semicolon: Option<String>
}

impl BlockExpression {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut vasm = vasm![];
        let mut ok = true;

        for (i, item) in self.list.iter().enumerate() {
            let is_last = i == self.list.len() - 1;
            let hint = match is_last {
                true => type_hint,
                false => None,
            };

            if let Some(mut item_vasm) = item.expression.process(hint, context) {
                if !is_last || item.semicolon.is_some() {
                    item_vasm.extend(vasm![VI::Drop(item_vasm.ty.clone())]);
                }

                // if !is_last && item.semicolon.is_none() {
                //     context.errors.add(&item.location.get_end(), format!("missing `;`"));
                // }

                vasm.extend(item_vasm);
            } else if is_last {
                ok = false
            }
        }

        match ok {
            true => Some(vasm),
            false => None,
        }
    }
}