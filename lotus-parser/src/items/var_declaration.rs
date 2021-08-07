use parsable::parsable;

use crate::program::{ProgramContext, Wasm};

use super::{Expression, Identifier, FullType, VarDeclarationQualifier};

#[parsable]
pub struct VarDeclaration {
    pub qualifier: Option<VarDeclarationQualifier>,
    pub var_type: FullType,
    pub var_name: Identifier,
    #[parsable(prefix="=")]
    pub init_value: Expression
}

impl VarDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        if context.inside_const_expr && self.qualifier.is_none() {
            context.error(self, format!("global variables must be declared with the `const` qualifier"));
        } else if !context.inside_const_expr && self.qualifier.is_some() {
            context.error(self, format!("local variables must be declared without the `const` qualifier"));
        }

        let var_exists = context.var_exists(&self.var_name);

        if var_exists {
            context.error(&self.var_name, format!("duplicate variable declaration: `{}` already exists in this scope", &self.var_name));
        }

        None
    }
}