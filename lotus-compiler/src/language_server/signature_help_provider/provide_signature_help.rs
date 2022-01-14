use crate::{program::{ProgramContext, FunctionCall, SELF_VAR_NAME, VariableInfo}, language_server::{LanguageServerCommandParameters, LanguageServerCommandOutput}};

pub fn provide_signature_help(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some(area) = context.signature_help_provider.get_area_under_cursor(&parameters.file_path, parameters.cursor_index) {
        let active_argument_index = area.get_active_argument_index(&parameters.file_path, parameters.cursor_index).unwrap_or(-1);

        // dbg!(&area.argument_locations);

        let mut label = format!("fn {}(", &area.function_name);
        let mut argument_ranges = vec![];
        let mut return_type_string = String::new();
        
        match &area.function_call {
            FunctionCall::Named(details) => {
                details.function.with_ref(|function_unwrapped| {
                    let arguments : Vec<&VariableInfo> = function_unwrapped.argument_variables.iter().filter(|var| var.name().as_str() != SELF_VAR_NAME).collect();

                    for (i, arg_info) in arguments.iter().enumerate() {
                        let arg_str = format!("{}: {}", arg_info.name().as_str(), arg_info.with_ref(|info| info.ty.to_string()));

                        argument_ranges.push((label.len(), label.len() + arg_str.len()));
                        label.push_str(&arg_str);

                        if i != arguments.len() - 1 {
                            label.push_str(", ");
                        }
                    }

                    if !function_unwrapped.signature.return_type.is_void() {
                        return_type_string = format!(" -> {}", function_unwrapped.signature.return_type.to_string());
                    }
                });
            },
            FunctionCall::Anonymous(details) => {
                for (i, ty) in details.signature.argument_types.iter().enumerate() {
                    let arg_str = format!("{}", ty.to_string());

                    argument_ranges.push((label.len(), label.len() + arg_str.len()));
                    label.push_str(&arg_str);

                    if i != details.signature.argument_types.len() - 1 {
                        label.push_str(", ");
                    }
                }

                if !details.signature.return_type.is_void() {
                    return_type_string = format!(" -> {}", details.signature.return_type.to_string());
                }
            },
        };

        label.push_str(")");
        label.push_str(&return_type_string);

        output.line("signature")
            .push(label)
            .push(active_argument_index);
        
        for (start, end) in argument_ranges {
            output.push(format!("{}:{}", start, end));
        }
    }
}