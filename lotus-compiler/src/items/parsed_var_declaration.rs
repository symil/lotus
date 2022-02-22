use std::{collections::HashMap, rc::Rc};
use parsable::{parsable, ItemLocation};
use crate::{program::{ProgramContext, TUPLE_FIRST_ASSOCIATED_TYPE_NAME, TUPLE_FIRST_METHOD_NAME, TUPLE_SECOND_ASSOCIATED_TYPE_NAME, TUPLE_SECOND_METHOD_NAME, Type, VariableInfo, VariableKind, Vasm}};
use super::{ParsedExpression, Identifier, ParsedType, ParsedVarDeclarationQualifier, ParsedVarDeclarationNames, ParsedColonToken, ParsedEqualToken, unwrap_item, ParsedVarDeclarationType};

#[parsable(cascade = true)]
pub struct ParsedVarDeclaration {
    pub qualifier: ParsedVarDeclarationQualifier,
    pub var_names: Option<ParsedVarDeclarationNames>,
    pub var_type: ParsedVarDeclarationType,
    pub equal: Option<ParsedEqualToken>,
    pub init_value: Option<ParsedExpression>,
}

impl ParsedVarDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<(Vec<VariableInfo>, Vasm)> {
        let var_names = unwrap_item(&self.var_names, &self.qualifier, context)?;
        let required_type = self.var_type.process(context);
        let equal = unwrap_item(&self.equal, &self.var_type, context)?;
        let init_value = unwrap_item(&self.init_value, equal, context)?;
        let vasm = init_value.process(required_type.as_ref(), context).unwrap_or(context.vasm());

        var_names.process(required_type.as_ref(), vasm, Some(&init_value.location), context)
    }
}