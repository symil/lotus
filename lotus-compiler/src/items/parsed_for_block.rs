use std::borrow::Cow;
use parsable::parsable;
use crate::{program::{BuiltinInterface, BuiltinType, GET_AT_INDEX_FUNC_NAME, GET_ITERABLE_LEN_FUNC_NAME, GET_ITERABLE_PTR_FUNC_NAME, ITERABLE_ASSOCIATED_TYPE_NAME, ProgramContext, ScopeKind, Type, TypeIndex, VariableInfo, VariableKind, Vasm, Wat}, wat};
use super::{ParsedExpression, Identifier, ParsedBlockExpression, ParsedVarDeclarationNames};

#[parsable]
pub struct ParsedForBlock {
    #[parsable(prefix="for")]
    pub iterator: ForIterator,
    #[parsable(prefix="in", set_marker="no-object")]
    pub range_start: ParsedExpression,
    #[parsable(prefix="..", set_marker="no-object")]
    pub range_end: Option<ParsedExpression>,
    pub body: ParsedBlockExpression
}

#[parsable]
pub enum ForIterator {
    Item(ParsedVarDeclarationNames),
    IndexAndItem(ParsedIndexAndItem)
}

#[parsable]
pub struct ParsedIndexAndItem {
    #[parsable(prefix="[")]
    pub index_name: Identifier,
    #[parsable(prefix=",", suffix="]")]
    pub item_names: ParsedVarDeclarationNames
}

impl ParsedForBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let (index_var_name, item_var_names) = match &self.iterator {
            ForIterator::Item(item_names) => (Cow::Owned(Identifier::unique("index")), item_names),
            ForIterator::IndexAndItem(index_and_item) => (Cow::Borrowed(&index_and_item.index_name), &index_and_item.item_names),
        };

        context.push_scope(ScopeKind::Loop);

        let mut result = None;
        let range_start_vasm_opt = self.range_start.process(None, context);
        let range_end_vasm_opt = self.range_end.as_ref().and_then(|expr| expr.process(None, context));
        let current_function_level = Some(context.get_function_level());

        if let Some(range_end) = &self.range_end {
            if let (Some(range_start_vasm), Some(range_end_vasm)) = (range_start_vasm_opt, range_end_vasm_opt) {
                if !range_start_vasm.ty.is_int() {
                    context.errors.type_mismatch(&self.range_start, &context.int_type(), &range_start_vasm.ty);
                }

                if !range_end_vasm.ty.is_int() {
                    context.errors.type_mismatch(range_end, &context.int_type(), &range_end_vasm.ty);
                }

                let index_var = VariableInfo::tmp("index", context.int_type());
                let item_var = VariableInfo::tmp("item", context.int_type());
                let range_end_var = VariableInfo::tmp("range_end", context.int_type());
                let declared_index_var = context.declare_local_variable(index_var_name.as_ref().clone(), context.int_type());
                let iteration_vasm = context.vasm()
                    .raw(Wat::get_local(&item_var.wasm_name()))
                    .set_type(context.int_type());

                if let Some((item_variables, init_vasm)) = item_var_names.process(None, iteration_vasm, Some(&self.range_start), context) {
                    if let Some(block_vasm) = self.body.process(None, context) {
                        if !block_vasm.ty.is_void() {
                            context.errors.type_mismatch(&self.body, &context.void_type(), &block_vasm.ty);
                        }

                        let index_var_wasm_name = index_var.get_wasm_name();
                        let item_var_wasm_name = item_var.get_wasm_name();
                        let range_end_var_wasm_name = range_end_var.get_wasm_name();
                        let content = context.vasm()
                            .declare_variable(&declared_index_var)
                            .declare_variable(&index_var)
                            .declare_variable(&item_var)
                            .declare_variable(&range_end_var)
                            .append(range_end_vasm)
                            .set_tmp_var(&range_end_var)
                            .append(range_start_vasm)
                            .set_tmp_var(&item_var)
                            .int(-1i32)
                            .set_tmp_var(&index_var)
                            .raw(Wat::increment_local_i32(&item_var_wasm_name, -1i32))
                            .block(context.vasm()
                                .loop_(context.vasm()
                                    .raw(Wat::increment_local_i32(&index_var_wasm_name, 1i32))
                                    .raw(Wat::increment_local_i32(&item_var_wasm_name, 1i32))
                                    .raw(wat!["i32.lt_s", Wat::get_local(&item_var_wasm_name), Wat::get_local(&range_end_var_wasm_name)])
                                    .jump_if(1, context.vasm().eqz())
                                    .init_var(&declared_index_var)
                                    .set_var(&declared_index_var, current_function_level, context.vasm()
                                        .raw(Wat::get_local(&index_var_wasm_name))
                                    )
                                    .append(init_vasm)
                                    .append(block_vasm)
                                    .jump(0)
                                )
                            )
                            .set_type(context.void_type());
                        
                        result = Some(content);
                    }
                }
            }
        } else if let Some(iterable_vasm) = range_start_vasm_opt {
            let required_interface_wrapped = context.get_builtin_interface(BuiltinInterface::Iterable);
            let item_type = iterable_vasm.ty.get_associated_type(ITERABLE_ASSOCIATED_TYPE_NAME).unwrap_or(Type::undefined());
            let pointer_type = context.get_builtin_type(BuiltinType::Pointer, vec![item_type.clone()]);

            let iterable_var = VariableInfo::tmp("iterable", context.int_type());
            let iterable_len_var = VariableInfo::tmp("iterable_len", context.int_type());
            let iterable_ptr_var = VariableInfo::tmp("iterable_ptr", context.int_type());
            let index_var = VariableInfo::tmp("index", context.int_type());
            let item_var = VariableInfo::tmp("item", item_type.clone());
            let declared_index_var = context.declare_local_variable(index_var_name.as_ref().clone(), context.int_type());
            let iteration_vasm = context.vasm()
                .raw(Wat::get_local(&item_var.wasm_name()))
                .set_type(&item_type);

            if let Some((item_variables, init_vasm)) = item_var_names.process(None, iteration_vasm, Some(&self.range_start), context) {
                if iterable_vasm.ty.check_match_interface(&required_interface_wrapped, &self.range_start, context) {
                    if let Some(block_vasm) = self.body.process(None, context) {
                        if !block_vasm.ty.is_void() {
                            context.errors.type_mismatch(&self.body, &context.void_type(), &block_vasm.ty);
                        }

                        let iterable_type = iterable_vasm.ty.clone();
                        let index_var_wasm_name = index_var.get_wasm_name();
                        let item_var_wasm_name = item_var.get_wasm_name();
                        let iterable_len_var_wasm_name = iterable_len_var.get_wasm_name();
                        let content = context.vasm()
                            .declare_variable(&declared_index_var)
                            .declare_variable(&iterable_var)
                            .declare_variable(&iterable_len_var)
                            .declare_variable(&iterable_ptr_var)
                            .declare_variable(&index_var)
                            .declare_variable(&item_var)
                            .append(iterable_vasm)
                            .set_tmp_var(&iterable_var)
                            .int(-1i32)
                            .set_tmp_var(&index_var)
                            .call_regular_method(&iterable_type, GET_ITERABLE_LEN_FUNC_NAME, &[], vec![context.vasm().get_tmp_var(&iterable_var)], context)
                            .set_tmp_var(&iterable_len_var)
                            .call_regular_method(&iterable_type, GET_ITERABLE_PTR_FUNC_NAME, &[], vec![context.vasm().get_tmp_var(&iterable_var)], context)
                            .set_tmp_var(&iterable_ptr_var)
                            .block(context.vasm()
                                .loop_(context.vasm()
                                    .raw(Wat::increment_local_i32(&index_var_wasm_name, 1i32))
                                    .raw(wat!["i32.lt_s", Wat::get_local(&index_var_wasm_name), Wat::get_local(&iterable_len_var_wasm_name)])
                                    .jump_if(1, context.vasm().eqz())
                                    .call_regular_method(&pointer_type, GET_AT_INDEX_FUNC_NAME, &[], vec![
                                        context.vasm().get_tmp_var(&iterable_ptr_var),
                                        context.vasm().get_tmp_var(&index_var)
                                    ], context)
                                    .set_tmp_var(&item_var)
                                    .init_var(&declared_index_var)
                                    .set_var(&declared_index_var, current_function_level,
                                        context.vasm().raw(Wat::get_local(&index_var_wasm_name))
                                    )
                                    .append(init_vasm)
                                    .append(block_vasm)
                                    .jump(0)
                                )
                            )
                            .set_type(context.void_type());

                        result = Some(content);
                    }
                }
            }
        }

        context.pop_scope();

        result
    }
}