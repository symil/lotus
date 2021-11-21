use parsable::parsable;
use crate::{program::{BuiltinType, NEW_METHOD_NAME, ProgramContext, Type, VI, Vasm}, vasm};
use super::{Expression, Identifier};

#[parsable]
pub struct ParenthesizedExpression {
    #[parsable(brackets="()",separator=",")]
    pub expr_list: Vec<Expression>
}

impl ParenthesizedExpression {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        for expr in &self.expr_list {
            expr.collected_instancied_type_names(list);
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self.expr_list.len() {
            0 => context.errors.add_and_none(self, format!("invalid empty expression")),
            1 => self.expr_list.first().unwrap().process(type_hint, context),
            2 => {
                let mut vasm_list = vec![];
                let mut type_hints = [None, None];

                if let Some(hint) = type_hint {
                    if let Some(first) = hint.get_associated_type("First") {
                        type_hints[0] = Some(first);
                    }

                    if let Some(second) = hint.get_associated_type("Second") {
                        type_hints[1] = Some(second);
                    }
                }

                for (expr, hint) in self.expr_list.iter().zip(type_hints.iter()) {
                    if let Some(vasm) = expr.process(hint.as_ref(), context)  {
                        vasm_list.push(vasm);
                    }
                }

                match vasm_list.len() {
                    2 => {
                        let final_type = context.get_builtin_type(BuiltinType::Pair, vec![vasm_list[0].ty.clone(), vasm_list[1].ty.clone()]);
                        let instruction = VI::call_static_method(&final_type, NEW_METHOD_NAME, &[], Vasm::merge(vasm_list), context);

                        Some(Vasm::new(final_type, vec![], vec![instruction]))
                    },
                    _ => None
                }
            },
            _ => context.errors.add_and_none(self, format!("tuples can only contain 2 values for now")),
        }
    }
}