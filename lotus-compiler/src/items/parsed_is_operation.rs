use parsable::{ItemLocation, parsable};
use crate::program::{ProgramContext, Vasm, Type, IS_METHOD_NAME, VariableInfo, BuiltinInterface, IS_SAME_TYPE_FUNCTION_NAME, TYPE_ID_METHOD_NAME};
use super::{ParsedType, Identifier, ParsedVarDeclarationNames, unwrap_item};

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
                Some(ty) => (ty, &parsed_type.location),
                None => {
                    return None;
                },
            },
            None => {
                context.errors.expected_identifier(&self.keyword);
                return None;
            },
        };

        let var_name = unwrap_item(&self.var_name, self, context)?;
        let is_source_object = source_type.match_builtin_interface(BuiltinInterface::Object, context);
        let is_target_object = target_type.match_builtin_interface(BuiltinInterface::Object, context);
        let both_object = is_source_object && is_target_object;

        let tmp_var_info = VariableInfo::tmp("is_tmp", source_type.clone());
        let assigned_vasm = context.vasm().get_tmp_var(&tmp_var_info);
        let access_level = context.get_function_level();

        match var_name.process(Some(&target_type), assigned_vasm, None, context) {
            // TODO: init variable only if the `is` operation returns true
            Some((_, init_var_vasm)) => {
                let mut vasm = context.vasm()
                    .declare_variable(&tmp_var_info)
                    .set_tmp_var(&tmp_var_info);
                
                if both_object {
                    vasm = vasm.call_static_method(&target_type, IS_METHOD_NAME, &[], vec![
                        context.vasm().get_tmp_var(&tmp_var_info)
                    ], context);
                } else {
                    vasm = vasm.call_sys_function(IS_SAME_TYPE_FUNCTION_NAME, &[], vec![
                        context.vasm().call_static_method(source_type, TYPE_ID_METHOD_NAME, &[], vec![], context),
                        context.vasm().call_static_method(&target_type, TYPE_ID_METHOD_NAME, &[], vec![], context),
                    ], context);
                }

                vasm = vasm.append(init_var_vasm)
                    .set_type(context.bool_type());

                Some(vasm)
            },
            None => None,
        }
    }
}