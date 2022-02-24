mod parsed_var_path {
    use parsable::parsable;
    use crate::{
        program::{AccessType, FieldKind, ProgramContext, Type, Vasm},
    };
    use super::{Identifier, ParsedVarPathRoot, ParsedVarPathSegment};
    pub struct ParsedVarPath {
        pub location: parsable::ItemLocation,
        pub root: Box<ParsedVarPathRoot>,
        pub path: Vec<ParsedVarPathSegment>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ParsedVarPath {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ParsedVarPath {
                    location: ref __self_0_0,
                    root: ref __self_0_1,
                    path: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ParsedVarPath");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "location",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "root",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "path",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl parsable::Parsable for ParsedVarPath {
        fn parse_item(reader__: &mut parsable::StringReader) -> Option<Self> {
            let start_index__ = reader__.get_index();
            let mut field_index__: usize = 0;
            let mut field_failed__ = false;
            let mut prefix_ok__ = true;
            let mut option_failed__ = false;
            let no_function_call_id = reader__.declare_marker("no-function-call");
            field_failed__ = false;
            prefix_ok__ = true;
            field_index__ = reader__.get_index();
            let mut root =
                match <Box<ParsedVarPathRoot> as parsable::Parsable>::parse_item(reader__) {
                    Some(value) => value,
                    None => {
                        reader__.set_expected_item::<Box<ParsedVarPathRoot>>();
                        reader__.set_index(start_index__);
                        return None;
                    }
                };
            reader__.eat_spaces();
            field_failed__ = false;
            prefix_ok__ = true;
            field_index__ = reader__.get_index();
            let mut path =
                match <Vec<ParsedVarPathSegment> as parsable::Parsable>::parse_item(reader__) {
                    Some(value) => value,
                    None => {
                        reader__.set_expected_item::<Vec<ParsedVarPathSegment>>();
                        reader__.set_index(start_index__);
                        return None;
                    }
                };
            reader__.eat_spaces();
            reader__.remove_marker(no_function_call_id);
            let location = reader__.get_item_location(start_index__);
            Some(Self {
                root,
                path,
                location,
            })
        }
        fn get_item_name() -> String {
            "ParsedVarPath".to_string()
        }
        fn location(&self) -> &parsable::ItemLocation {
            &self.location
        }
    }
    impl std::ops::Deref for ParsedVarPath {
        type Target = parsable::ItemLocation;
        fn deref(&self) -> &parsable::ItemLocation {
            <Self as parsable::Parsable>::location(self)
        }
    }
    impl ParsedVarPath {
        pub fn collect_instancied_type_names(
            &self,
            list: &mut Vec<String>,
            context: &mut ProgramContext,
        ) {
            self.root.collect_instancied_type_names(list, context);
        }
        pub fn process(
            &self,
            type_hint: Option<&Type>,
            access_type: AccessType,
            context: &mut ProgramContext,
        ) -> Option<Vasm> {
            let mut current_access_type = match self.path.is_empty() {
                true => access_type,
                false => AccessType::Get,
            };
            let mut parent_type = Type::undefined();
            let mut result = context.vasm();
            let mut current_type_hint = match self.path.is_empty() {
                true => type_hint,
                false => None,
            };
            if let Some(root_vasm) =
                self.root
                    .process(current_type_hint, current_access_type, context)
            {
                parent_type = root_vasm.ty.clone();
                result = result.append(root_vasm);
                for (i, segment) in self.path.iter().enumerate() {
                    if i == self.path.len() - 1 {
                        current_access_type = access_type;
                        current_type_hint = type_hint;
                    }
                    if let Some(segment_vasm) = segment.process(
                        &parent_type,
                        current_type_hint,
                        current_access_type,
                        context,
                    ) {
                        parent_type = segment_vasm.ty.clone();
                        result = result.append(segment_vasm);
                    } else {
                        return None;
                    }
                }
            }
            Some(result)
        }
    }
}
