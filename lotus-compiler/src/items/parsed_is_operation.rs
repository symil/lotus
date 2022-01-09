use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Vasm, Type, IS_METHOD_NAME, VariableInfo};
use super::{ParsedType, Identifier, ParsedVarDeclarationNames};

#[parsable]
pub struct ParsedIsOperation {
    pub keyword: ParsedIsKeyword,
    pub ty: Option<ParsedType>,
    #[parsable(brackets="()")]
    pub var_name: Option<ParsedVarDeclarationNames>
}

#[parsable]
pub struct ParsedIsKeyword {
    #[parsable(value="is")]
    pub token: String
}

impl ParsedIsOperation {
    pub fn process(&self, source_type: &Type, context: &mut ProgramContext) -> Option<Vasm> {
        let (target_type, type_location) = match &self.ty {
            Some(parsed_type) => match parsed_type.process(true, context) {
                Some(ty) => match ty.is_object() {
                    true => (ty, &parsed_type.location),
                    false => {
                        context.errors.expected_class_type(parsed_type, &ty);
                        return None;
                    },
                },
                None => {
                    return None;
                },
            },
            None => {
                context.errors.expected_identifier(&self.keyword);
                return None;
            },
        };

        let var_name = match &self.var_name {
            Some(name) => name,
            None => {
                context.errors.expected_identifier(type_location);
                return None;
            },
        };

        let tmp_var_info = VariableInfo::tmp("is_tmp", source_type.clone());
        let assigned_vasm = context.vasm().get_tmp_var(&tmp_var_info);
        let access_level = context.get_function_level();

        match var_name.process(Some(&target_type), assigned_vasm, None, context) {
            // TODO: init variable only if the `is` operation returns true
            Some((_, init_var_vasm)) => Some(context.vasm()
                .declare_variable(&tmp_var_info)
                .tee_tmp_var(&tmp_var_info)
                .call_static_method(&target_type, IS_METHOD_NAME, &[], vec![], context)
                .append(init_var_vasm)
                .set_type(context.bool_type())
            ),
            None => None,
        }
    }
}