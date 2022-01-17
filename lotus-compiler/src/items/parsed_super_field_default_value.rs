use parsable::parsable;
use crate::program::{ProgramContext, Type, Vasm};
use super::{ParsedSuperKeyword, ParsedDot, Identifier, ParsedEqual, ParsedExpression, unwrap_item, ParsedComma};

#[parsable(cascade = true)]
pub struct ParsedSuperFieldDefaultValue {
    pub super_keyword: ParsedSuperKeyword,
    pub dot: Option<ParsedDot>,
    pub name: Option<Identifier>,
    pub equal: Option<ParsedEqual>,
    pub expression: Option<ParsedExpression>,
    #[parsable(cascade = false)]
    pub comma: Option<ParsedComma>,
}

impl ParsedSuperFieldDefaultValue {
    pub fn process(&self, self_type: &Type, context: &mut ProgramContext) -> Option<(String, Vasm)> {
        let dot = unwrap_item(&self.dot, self, context)?;
        let name = unwrap_item(&self.name, self, context)?;
        let equal = unwrap_item(&self.equal, self, context)?;
        let expression = unwrap_item(&self.expression, self, context)?;
        let field_type = match self_type.get_field(name.as_str()) {
            Some(field_info) => field_info.ty.clone(),
            None => {
                context.errors.generic(name, format!("type `{}` has no field `{}`", self_type, name.as_str()));
                return None;
            },
        };
        let vasm = match expression.process(Some(&field_type), context) {
            Some(vasm) => match vasm.ty.is_assignable_to(&field_type) {
                true => vasm,
                false => {
                    context.errors.type_mismatch(expression, &field_type, &vasm.ty);
                    return None;
                },
            },
            None => return None,
        };

        Some((name.to_string(), vasm))
    }
}