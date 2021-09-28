use std::rc::Rc;

use parsable::parsable;
use crate::{program::{BuiltinInterface, BuiltinType, GET_AT_INDEX_FUNC_NAME, GET_ITERABLE_LEN_FUNC_NAME, GET_ITERABLE_PTR_FUNC_NAME, ITERABLE_ASSOCIATED_TYPE_NAME, ProgramContext, ScopeKind, Type, VI, VariableInfo, VariableKind, Vasm, Wat}, vasm, wat};
use super::{Expression, Identifier, Statement, StatementList};

#[parsable]
pub struct ForBlock {
    #[parsable(prefix="for")]
    pub iterator: ForIterator,
    #[parsable(prefix="in", set_marker="no-object")]
    pub range_start: Expression,
    #[parsable(prefix="..", set_marker="no-object")]
    pub range_end: Option<Expression>,
    pub statements: StatementList
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
        let return_found = context.return_found;

        context.ckeck_var_unicity(&index_var_name);
        context.ckeck_var_unicity(&item_var_name);
        context.push_scope(ScopeKind::Loop);

        let mut result = None;
        let range_start_vasm_opt = self.range_start.process(None, context);
        let range_end_vasm_opt = self.range_end.as_ref().and_then(|expr| expr.process(None, context));

        if let Some(range_end) = &self.range_end {
            if let (Some(range_start_vasm), Some(range_end_vasm)) = (range_start_vasm_opt, range_end_vasm_opt) {
                if !range_start_vasm.ty.is_int() {
                    context.errors.add(&self.range_start, format!("range start: expected `{}`, got `{}`", context.int_type(), &range_start_vasm.ty));
                }

                if !range_end_vasm.ty.is_int() {
                    context.errors.add(range_end, format!("range end: expected `{}`, got `{}`", context.int_type(), &range_end_vasm.ty));
                }

                let range_end_var_name = Identifier::unique("range_end", self);

                let index_var = VariableInfo::new(index_var_name.clone(), context.int_type(), VariableKind::Local);
                let item_var = VariableInfo::new(item_var_name.clone(), context.int_type(), VariableKind::Local);
                let range_end_var = VariableInfo::new(range_end_var_name.clone(), context.int_type(), VariableKind::Local);
                let variables = vec![Rc::clone(&index_var), Rc::clone(&item_var), Rc::clone(&range_end_var)];

                context.push_var(&index_var);
                context.push_var(&item_var);

                if let Some(block_vasm) = self.statements.process(context) {
                    result = Some(Vasm::new(Type::Undefined, variables, vec![
                        VI::set(&range_end_var, range_end_vasm),
                        VI::set(&item_var, range_start_vasm),
                        VI::set(&index_var, VI::int(-1)),
                        VI::raw(Wat::increment_local_i32(&item_var.wasm_name, -1)),
                        VI::block(vasm![
                            VI::loop_(vasm![
                                VI::raw(Wat::increment_local_i32(&index_var.wasm_name, 1)),
                                VI::raw(Wat::increment_local_i32(&item_var.wasm_name, 1)),
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

            let iterable_var = VariableInfo::new(Identifier::unique("iterable", self), context.int_type(), VariableKind::Local);
            let iterable_len_var = VariableInfo::new(Identifier::unique("iterable_len", self), context.int_type(), VariableKind::Local);
            let iterable_ptr_var = VariableInfo::new(Identifier::unique("iterable_ptr", self), context.int_type(), VariableKind::Local);
            let index_var = VariableInfo::new(index_var_name, context.int_type(), VariableKind::Local);
            let item_var = VariableInfo::new(item_var_name, item_type.clone(), VariableKind::Local);
            let variables = vec![Rc::clone(&iterable_var), Rc::clone(&iterable_len_var), Rc::clone(&iterable_ptr_var), Rc::clone(&index_var), Rc::clone(&item_var)];

            context.push_var(&index_var);
            context.push_var(&item_var);

            if iterable_vasm.ty.check_match_interface(&required_interface_wrapped, &self.range_start, context) {
                if let Some(block_vasm) = self.statements.process(context) {
                    let iterable_type = iterable_vasm.ty.clone();

                    result = Some(Vasm::new(Type::Undefined, variables, vec![
                        VI::set(&iterable_var, iterable_vasm),
                        VI::set(&index_var, VI::int(-1)),
                        VI::set(&iterable_len_var, VI::call_method(&iterable_type, iterable_type.get_regular_method(GET_ITERABLE_LEN_FUNC_NAME).unwrap(), &[], vec![VI::get(&iterable_var)])),
                        VI::set(&iterable_ptr_var, VI::call_method(&iterable_type, iterable_type.get_regular_method(GET_ITERABLE_PTR_FUNC_NAME).unwrap(), &[], vec![VI::get(&iterable_var)])),
                        VI::block(vec![
                            VI::loop_(vasm![
                                VI::raw(Wat::increment_local_i32(&index_var.wasm_name, 1)),
                                VI::raw(wat!["i32.lt_s", index_var.get_to_stack(), iterable_len_var.get_to_stack()]),
                                VI::jump_if(1, VI::raw(wat!["i32.eqz"])),
                                VI::set(&item_var, VI::call_method(&pointer_type, pointer_type.get_static_method(GET_AT_INDEX_FUNC_NAME).unwrap(), &[], vec![VI::get(&iterable_ptr_var), VI::get(&index_var)])),
                                block_vasm,
                                VI::jump(0)
                            ])
                        ])
                    ]));
                }
            } else {
                self.statements.process(context);
            }
        } else {
            self.statements.process(context);
        }

        context.pop_scope();
        context.return_found = return_found;

        result
    }
}