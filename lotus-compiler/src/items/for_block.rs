use std::rc::Rc;

use parsable::parsable;
use crate::{program::{BuiltinInterface, BuiltinType, GET_AT_INDEX_FUNC_NAME, GET_ITERABLE_LEN_FUNC_NAME, GET_ITERABLE_PTR_FUNC_NAME, ITERABLE_ASSOCIATED_TYPE_NAME, ProgramContext, ScopeKind, Type, TypeIndex, VI, VariableInfo, VariableKind, Vasm, Wat}, vasm, wat};
use super::{Expression, Identifier, BlockExpression};

#[parsable]
pub struct ForBlock {
    #[parsable(prefix="for")]
    pub iterator: ForIterator,
    #[parsable(prefix="in", set_marker="no-object")]
    pub range_start: Expression,
    #[parsable(prefix="..", set_marker="no-object")]
    pub range_end: Option<Expression>,
    pub body: BlockExpression
}

#[parsable]
pub enum ForIterator {
    Item(Identifier),
    IndexAndItem(IndexAndItem)
}

#[parsable]
pub struct IndexAndItem {
    #[parsable(prefix="(")]
    pub index_name: Identifier,
    #[parsable(prefix=",", suffix=")")]
    pub item_name: Identifier
}

impl ForBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let (index_var_name, item_var_name) = match &self.iterator {
            ForIterator::Item(item_name) => (Identifier::unique("index", self), item_name.clone()),
            ForIterator::IndexAndItem(index_and_item) => (index_and_item.index_name.clone(), index_and_item.item_name.clone()),
        };

        context.check_var_unicity(&index_var_name);
        context.check_var_unicity(&item_var_name);
        context.push_scope(ScopeKind::Loop);

        let mut result = None;
        let range_start_vasm_opt = self.range_start.process(None, context);
        let range_end_vasm_opt = self.range_end.as_ref().and_then(|expr| expr.process(None, context));

        if let Some(range_end) = &self.range_end {
            if let (Some(range_start_vasm), Some(range_end_vasm)) = (range_start_vasm_opt, range_end_vasm_opt) {
                if !range_start_vasm.ty.is_int() {
                    context.errors.add(&self.range_start, format!("expected `{}`, got `{}`", context.int_type(), &range_start_vasm.ty));
                }

                if !range_end_vasm.ty.is_int() {
                    context.errors.add(range_end, format!("expected `{}`, got `{}`", context.int_type(), &range_end_vasm.ty));
                }

                let range_end_var_name = Identifier::unique("range_end", self);

                let index_var = VariableInfo::from(index_var_name.clone(), context.int_type(), VariableKind::Local);
                let item_var = VariableInfo::from(item_var_name.clone(), context.int_type(), VariableKind::Local);
                let range_end_var = VariableInfo::from(range_end_var_name.clone(), context.int_type(), VariableKind::Local);
                let variables = vec![index_var.clone(), item_var.clone(), range_end_var.clone()];

                context.push_var(&index_var);
                context.push_var(&item_var);

                if let Some(block_vasm) = self.body.process(None, context) {
                    if !block_vasm.ty.is_void() {
                        context.errors.add(&self.body, format!("expected `{}`, got `{}`", Type::Void, &block_vasm.ty));
                    }

                    result = Some(Vasm::new(Type::Void, variables, vec![
                        VI::set_var(&range_end_var, range_end_vasm),
                        VI::set_var(&item_var, range_start_vasm),
                        VI::set_var(&index_var, VI::int(-1)),
                        VI::raw(Wat::increment_local_i32(&item_var.get_wasm_name(), -1)),
                        VI::block(vasm![
                            VI::loop_(vasm![
                                VI::raw(Wat::increment_local_i32(&index_var.get_wasm_name(), 1)),
                                VI::raw(Wat::increment_local_i32(&item_var.get_wasm_name(), 1)),
                                VI::raw(wat!["i32.lt_s", item_var.get_to_stack(), range_end_var.get_to_stack()]),
                                VI::jump_if(1, VI::raw(wat!["i32.eqz"])),
                                block_vasm,
                                VI::jump(0)
                            ])
                        ])
                    ]));
                }
            }
        } else if let Some(iterable_vasm) = range_start_vasm_opt {
            let required_interface_wrapped = context.get_builtin_interface(BuiltinInterface::Iterable);
            let item_type = iterable_vasm.ty.get_associated_type(ITERABLE_ASSOCIATED_TYPE_NAME).unwrap_or(Type::Undefined);
            let pointer_type = context.get_builtin_type(BuiltinType::Pointer, vec![item_type.clone()]);

            let iterable_var = VariableInfo::from(Identifier::unique("iterable", self), context.int_type(), VariableKind::Local);
            let iterable_len_var = VariableInfo::from(Identifier::unique("iterable_len", self), context.int_type(), VariableKind::Local);
            let iterable_ptr_var = VariableInfo::from(Identifier::unique("iterable_ptr", self), context.int_type(), VariableKind::Local);
            let index_var = VariableInfo::from(index_var_name, context.int_type(), VariableKind::Local);
            let item_var = VariableInfo::from(item_var_name, item_type.clone(), VariableKind::Local);
            let variables = vec![iterable_var.clone(), iterable_len_var.clone(), iterable_ptr_var.clone(), index_var.clone(), item_var.clone()];

            context.push_var(&index_var);
            context.push_var(&item_var);

            if iterable_vasm.ty.check_match_interface(&required_interface_wrapped, &self.range_start, context) {
                if let Some(block_vasm) = self.body.process(None, context) {
                    if !block_vasm.ty.is_void() {
                        context.errors.add(&self.body, format!("expected `{}`, got `{}`", Type::Void, &block_vasm.ty));
                    }

                    let iterable_type = iterable_vasm.ty.clone();

                    result = Some(Vasm::new(Type::Void, variables, vec![
                        VI::set_var(&iterable_var, iterable_vasm),
                        VI::set_var(&index_var, VI::int(-1)),
                        VI::set_var(&iterable_len_var, VI::call_regular_method(&iterable_type, GET_ITERABLE_LEN_FUNC_NAME, &[], vec![VI::get_var(&iterable_var)], context)),
                        VI::set_var(&iterable_ptr_var, VI::call_regular_method(&iterable_type, GET_ITERABLE_PTR_FUNC_NAME, &[], vec![VI::get_var(&iterable_var)], context)),
                        VI::block(vec![
                            VI::loop_(vasm![
                                VI::raw(Wat::increment_local_i32(&index_var.get_wasm_name(), 1)),
                                VI::raw(wat!["i32.lt_s", index_var.get_to_stack(), iterable_len_var.get_to_stack()]),
                                VI::jump_if(1, VI::raw(wat!["i32.eqz"])),
                                VI::set_var(&item_var, VI::call_regular_method(&pointer_type, GET_AT_INDEX_FUNC_NAME, &[], vec![VI::get_var(&iterable_ptr_var), VI::get_var(&index_var)], context)),
                                block_vasm,
                                VI::jump(0)
                            ])
                        ])
                    ]));
                }
            }
        }

        context.pop_scope();

        result
    }
}