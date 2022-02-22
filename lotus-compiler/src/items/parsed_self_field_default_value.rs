use parsable::{parsable, Token};
use crate::program::{ProgramContext, Type, Vasm, SELF_VAR_NAME};
use super::{ParsedDotToken, Identifier, ParsedEqualToken, ParsedExpression, unwrap_item, ParsedCommaToken, ParsedSemicolonToken};

#[parsable(cascade = true)]
pub struct ParsedSuperFieldDefaultValue {
    pub self_keyword: Token<SELF_VAR_NAME>,
    pub dot: Option<ParsedDotToken>,
    pub name: Option<Identifier>,
    pub equal: Option<ParsedEqualToken>,
    pub expression: Option<ParsedExpression>,
    #[parsable(cascade = false)]
    pub semicolon: Option<ParsedSemicolonToken>,
}

impl ParsedSuperFieldDefaultValue {
    pub fn process(&self, self_type: &Type, context: &mut ProgramContext) -> Option<(String, Vasm)> {
        let dot = unwrap_item(&self.dot, self, context)?;

        context.completion_provider.add_field_completion(dot, self_type, false, false, "");

        let name = unwrap_item(&self.name, self, context)?;

        context.completion_provider.add_field_completion(name, self_type, false, false, "");

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

        unwrap_item(&self.semicolon, self, context);

        Some((name.to_string(), vasm))
    }
}