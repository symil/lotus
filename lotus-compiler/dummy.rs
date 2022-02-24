mod parsed_var_path_root {
    use parsable::{ItemLocation, parsable};
    use crate::{
        program::{AccessType, ProgramContext, Type, VariableKind, Vasm},
    };
    use super::{
        ParsedAction, ParsedArrayLiteral, ParsedAssignment, ParsedBlockExpression,
        ParsedBooleanLiteral, ParsedCharLiteral, ParsedExpression, ParsedFieldOrMethodAccess,
        ParsedForBlock, ParsedAnonymousFunction, Identifier, ParsedIfBlock,
        ParsedIterAncestorsBlock, ParsedIterFieldsBlock, ParsedIterVariantsBlock, ParsedMatchBlock,
        ParsedNoneLiteral, ParsedNumberLiteral, ParsedObjectLiteral, ParsedParenthesizedExpression,
        ParsedStaticFieldOrMethod, ParsedStringLiteral, ParsedTemplateString, ParsedVarDeclaration,
        ParsedVarRef, ParsedWhileBlock, ParsedMacroExpression, ParsedPrefixedVarRef,
        ParsedMacroDebug,
    };
    pub enum ParsedVarPathRoot {
        Macro(ParsedMacroExpression),
        DebugMacro(ParsedMacroDebug),
        VarDeclaration(ParsedVarDeclaration),
        Action(ParsedAction),
        MatchBlock(ParsedMatchBlock),
        IfBlock(ParsedIfBlock),
        IterFields(ParsedIterFieldsBlock),
        IterVariants(ParsedIterVariantsBlock),
        IterAncestors(ParsedIterAncestorsBlock),
        WhileBlock(ParsedWhileBlock),
        ForBlock(ParsedForBlock),
        Block(ParsedBlockExpression),
        NoneLiteral(ParsedNoneLiteral),
        BooleanLiteral(ParsedBooleanLiteral),
        NumberLiteral(ParsedNumberLiteral),
        CharLiteral(ParsedCharLiteral),
        StringLiteral(ParsedStringLiteral),
        TemplateString(ParsedTemplateString),
        ArrayLiteral(ParsedArrayLiteral),
        StaticFieldOrMethod(ParsedStaticFieldOrMethod),
        ObjectLiteral(ParsedObjectLiteral),
        FunctionLiteral(ParsedAnonymousFunction),
        Parenthesized(ParsedParenthesizedExpression),
        PrefixedVarRef(ParsedPrefixedVarRef),
        VarRef(ParsedVarRef),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ParsedVarPathRoot {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ParsedVarPathRoot::Macro(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Macro");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::DebugMacro(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "DebugMacro");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::VarDeclaration(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "VarDeclaration");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::Action(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Action");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::MatchBlock(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "MatchBlock");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::IfBlock(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "IfBlock");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::IterFields(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "IterFields");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::IterVariants(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "IterVariants");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::IterAncestors(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "IterAncestors");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::WhileBlock(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "WhileBlock");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::ForBlock(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ForBlock");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::Block(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Block");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::NoneLiteral(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "NoneLiteral");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::BooleanLiteral(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "BooleanLiteral");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::NumberLiteral(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "NumberLiteral");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::CharLiteral(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "CharLiteral");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::StringLiteral(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "StringLiteral");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::TemplateString(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "TemplateString");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::ArrayLiteral(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ArrayLiteral");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::StaticFieldOrMethod(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "StaticFieldOrMethod");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::ObjectLiteral(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ObjectLiteral");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::FunctionLiteral(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "FunctionLiteral");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::Parenthesized(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Parenthesized");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::PrefixedVarRef(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "PrefixedVarRef");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ParsedVarPathRoot::VarRef(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "VarRef");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl parsable::Parsable for ParsedVarPathRoot {
        fn parse_item(reader__: &mut parsable::StringReader) -> Option<Self> {
            let start_index__ = reader__.get_index();
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedMacroExpression as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::Macro(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedMacroDebug as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::DebugMacro(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedVarDeclaration as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::VarDeclaration(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedAction as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::Action(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedMatchBlock as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::MatchBlock(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedIfBlock as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::IfBlock(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedIterFieldsBlock as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::IterFields(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedIterVariantsBlock as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::IterVariants(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedIterAncestorsBlock as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::IterAncestors(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedWhileBlock as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::WhileBlock(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedForBlock as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::ForBlock(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedBlockExpression as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::Block(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedNoneLiteral as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::NoneLiteral(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedBooleanLiteral as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::BooleanLiteral(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedNumberLiteral as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::NumberLiteral(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedCharLiteral as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::CharLiteral(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedStringLiteral as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::StringLiteral(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedTemplateString as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::TemplateString(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedArrayLiteral as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::ArrayLiteral(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedStaticFieldOrMethod as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::StaticFieldOrMethod(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (!reader__.get_marker("no-object")) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedObjectLiteral as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::ObjectLiteral(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedAnonymousFunction as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::FunctionLiteral(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
            }
            if (true) {
                let field_22_no_object_id = reader__.declare_marker("no-object");
                let field_22_no_function_call_value =
                    reader__.set_marker("no-function-call", false);
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedParenthesizedExpression as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            reader__.remove_marker(field_22_no_object_id);
                            return Some(Self::Parenthesized(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
                reader__.set_marker("no-function-call", field_22_no_function_call_value);
                reader__.remove_marker(field_22_no_object_id);
            }
            if (true) {
                let field_23_no_function_call_value =
                    reader__.set_marker("no-function-call", false);
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedPrefixedVarRef as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::PrefixedVarRef(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
                reader__.set_marker("no-function-call", field_23_no_function_call_value);
            }
            if (true) {
                let field_24_no_function_call_value =
                    reader__.set_marker("no-function-call", false);
                let prefix_ok__ = true;
                if prefix_ok__ {
                    if let Some(value_0) =
                        <ParsedVarRef as parsable::Parsable>::parse_item(reader__)
                    {
                        reader__.eat_spaces();
                        let suffix_ok__ = true;
                        if suffix_ok__ {
                            return Some(Self::VarRef(value_0));
                        }
                    }
                }
                reader__.set_index(start_index__);
                reader__.set_marker("no-function-call", field_24_no_function_call_value);
            }
            None
        }
        fn get_item_name() -> String {
            "ParsedVarPathRoot".to_string()
        }
        fn location(&self) -> &parsable::ItemLocation {
            match self {
                Self::Macro(value) => {
                    <ParsedMacroExpression as parsable::Parsable>::location(value)
                }
                Self::DebugMacro(value) => {
                    <ParsedMacroDebug as parsable::Parsable>::location(value)
                }
                Self::VarDeclaration(value) => {
                    <ParsedVarDeclaration as parsable::Parsable>::location(value)
                }
                Self::Action(value) => <ParsedAction as parsable::Parsable>::location(value),
                Self::MatchBlock(value) => {
                    <ParsedMatchBlock as parsable::Parsable>::location(value)
                }
                Self::IfBlock(value) => <ParsedIfBlock as parsable::Parsable>::location(value),
                Self::IterFields(value) => {
                    <ParsedIterFieldsBlock as parsable::Parsable>::location(value)
                }
                Self::IterVariants(value) => {
                    <ParsedIterVariantsBlock as parsable::Parsable>::location(value)
                }
                Self::IterAncestors(value) => {
                    <ParsedIterAncestorsBlock as parsable::Parsable>::location(value)
                }
                Self::WhileBlock(value) => {
                    <ParsedWhileBlock as parsable::Parsable>::location(value)
                }
                Self::ForBlock(value) => <ParsedForBlock as parsable::Parsable>::location(value),
                Self::Block(value) => {
                    <ParsedBlockExpression as parsable::Parsable>::location(value)
                }
                Self::NoneLiteral(value) => {
                    <ParsedNoneLiteral as parsable::Parsable>::location(value)
                }
                Self::BooleanLiteral(value) => {
                    <ParsedBooleanLiteral as parsable::Parsable>::location(value)
                }
                Self::NumberLiteral(value) => {
                    <ParsedNumberLiteral as parsable::Parsable>::location(value)
                }
                Self::CharLiteral(value) => {
                    <ParsedCharLiteral as parsable::Parsable>::location(value)
                }
                Self::StringLiteral(value) => {
                    <ParsedStringLiteral as parsable::Parsable>::location(value)
                }
                Self::TemplateString(value) => {
                    <ParsedTemplateString as parsable::Parsable>::location(value)
                }
                Self::ArrayLiteral(value) => {
                    <ParsedArrayLiteral as parsable::Parsable>::location(value)
                }
                Self::StaticFieldOrMethod(value) => {
                    <ParsedStaticFieldOrMethod as parsable::Parsable>::location(value)
                }
                Self::ObjectLiteral(value) => {
                    <ParsedObjectLiteral as parsable::Parsable>::location(value)
                }
                Self::FunctionLiteral(value) => {
                    <ParsedAnonymousFunction as parsable::Parsable>::location(value)
                }
                Self::Parenthesized(value) => {
                    <ParsedParenthesizedExpression as parsable::Parsable>::location(value)
                }
                Self::PrefixedVarRef(value) => {
                    <ParsedPrefixedVarRef as parsable::Parsable>::location(value)
                }
                Self::VarRef(value) => <ParsedVarRef as parsable::Parsable>::location(value),
            }
        }
    }
    impl std::ops::Deref for ParsedVarPathRoot {
        type Target = parsable::ItemLocation;
        fn deref(&self) -> &parsable::ItemLocation {
            <Self as parsable::Parsable>::location(self)
        }
    }
    impl ParsedVarPathRoot {
        pub fn as_str(&self) -> &'static str {
            match self {
                _ => "",
            }
        }
    }
    impl ParsedVarPathRoot {
        fn is_var_ref(&self) -> bool {
            match self {
                ParsedVarPathRoot::VarRef(_) => true,
                _ => false,
            }
        }
        pub fn collect_instancied_type_names(
            &self,
            list: &mut Vec<String>,
            context: &mut ProgramContext,
        ) {
            match self {
                ParsedVarPathRoot::Macro(_) => {}
                ParsedVarPathRoot::DebugMacro(_) => {}
                ParsedVarPathRoot::NoneLiteral(_) => {}
                ParsedVarPathRoot::BooleanLiteral(_) => {}
                ParsedVarPathRoot::NumberLiteral(_) => {}
                ParsedVarPathRoot::CharLiteral(_) => {}
                ParsedVarPathRoot::StringLiteral(_) => {}
                ParsedVarPathRoot::ArrayLiteral(array_literal) => {
                    array_literal.collect_instancied_type_names(list, context)
                }
                ParsedVarPathRoot::ObjectLiteral(object_literal) => {
                    object_literal.collect_instancied_type_names(list, context)
                }
                ParsedVarPathRoot::StaticFieldOrMethod(_) => {}
                ParsedVarPathRoot::Parenthesized(expr) => {
                    expr.collect_instancied_type_names(list, context)
                }
                ParsedVarPathRoot::VarRef(var_ref) => var_ref.collect_instancied_type_names(list),
                _ => ::core::panicking::panic("not yet implemented"),
            }
        }
        pub fn process(
            &self,
            type_hint: Option<&Type>,
            access_type: AccessType,
            context: &mut ProgramContext,
        ) -> Option<Vasm> {
            if let AccessType::Set(location) = access_type {
                if !self.is_var_ref() {
                    context.errors.generic(location, {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["invalid assignment"],
                            &[],
                        ));
                        res
                    });
                }
            }
            match self {
                ParsedVarPathRoot::Macro(mac) => mac.process(context),
                ParsedVarPathRoot::DebugMacro(mac) => mac.process(context),
                ParsedVarPathRoot::NoneLiteral(none_literal) => {
                    none_literal.process(type_hint, context)
                }
                ParsedVarPathRoot::BooleanLiteral(boolean_literal) => {
                    boolean_literal.process(context)
                }
                ParsedVarPathRoot::NumberLiteral(number_literal) => {
                    number_literal.process(type_hint, context)
                }
                ParsedVarPathRoot::CharLiteral(char_literal) => char_literal.process(context),
                ParsedVarPathRoot::StringLiteral(string_literal) => string_literal.process(context),
                ParsedVarPathRoot::TemplateString(template_string) => {
                    template_string.process(context)
                }
                ParsedVarPathRoot::ArrayLiteral(array_literal) => {
                    array_literal.process(type_hint, context)
                }
                ParsedVarPathRoot::ObjectLiteral(object_literal) => object_literal.process(context),
                ParsedVarPathRoot::StaticFieldOrMethod(static_field_or_method) => {
                    static_field_or_method.process(type_hint, context)
                }
                ParsedVarPathRoot::VarDeclaration(var_declaration) => {
                    var_declaration.process(context).map(|(_, vasm)| vasm)
                }
                ParsedVarPathRoot::Action(action) => action.process(context),
                ParsedVarPathRoot::IfBlock(if_block) => if_block.process(type_hint, context),
                ParsedVarPathRoot::IterFields(iter_fields) => iter_fields.process(context),
                ParsedVarPathRoot::IterVariants(iter_variants) => iter_variants.process(context),
                ParsedVarPathRoot::IterAncestors(iter_ancestors) => iter_ancestors.process(context),
                ParsedVarPathRoot::WhileBlock(while_block) => while_block.process(context),
                ParsedVarPathRoot::ForBlock(for_block) => for_block.process(context),
                ParsedVarPathRoot::MatchBlock(match_block) => {
                    match_block.process(type_hint, context)
                }
                ParsedVarPathRoot::Parenthesized(expr) => expr.process(type_hint, context),
                ParsedVarPathRoot::PrefixedVarRef(prefixed_var_ref) => {
                    prefixed_var_ref.process(type_hint, access_type, context)
                }
                ParsedVarPathRoot::VarRef(var_ref) => {
                    var_ref.process(type_hint, access_type, context)
                }
                ParsedVarPathRoot::Block(block) => block.process(type_hint, context),
                ParsedVarPathRoot::FunctionLiteral(function_literal) => {
                    function_literal.process(type_hint, context)
                }
            }
        }
    }
}
