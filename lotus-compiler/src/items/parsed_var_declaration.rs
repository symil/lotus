use std::{collections::HashMap, rc::Rc};
use parsable::{parsable, DataLocation};
use crate::{program::{ProgramContext, TUPLE_FIRST_ASSOCIATED_TYPE_NAME, TUPLE_FIRST_METHOD_NAME, TUPLE_SECOND_ASSOCIATED_TYPE_NAME, TUPLE_SECOND_METHOD_NAME, Type, VariableInfo, VariableKind, Vasm}};
use super::{ParsedExpression, Identifier, ParsedType, ParsedVarDeclarationQualifier, ParsedVarDeclarationNames};

#[parsable]
pub struct ParsedVarDeclaration {
    pub qualifier: ParsedVarDeclarationQualifier,
    pub var_names: ParsedVarDeclarationNames,
    #[parsable(prefix=":")]
    pub var_type: Option<ParsedType>,
    #[parsable(prefix="=")]
    pub init_value: Option<ParsedExpression>,
}

impl ParsedVarDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<(Vec<VariableInfo>, Vasm)> {
        let required_type = self.var_type.as_ref().and_then(|parsed_type| parsed_type.process(true, context));
        let (init_value, location) = match &self.init_value {
            Some(init_value) => (
                init_value.process(required_type.as_ref(), context).unwrap_or(context.vasm()),
                &init_value.location
            ),
            None => {
                context.errors.expected_expression(self);
                (context.vasm(), &self.location)
            },
        };

        self.var_names.process(required_type.as_ref(), init_value, Some(location), context)
    }
}