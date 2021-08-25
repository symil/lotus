use parsable::parsable;
use crate::{generation::{ToWat, ToWatVec, Wat}, program::{ARRAY_GET_BODY_FUNC_NAME, ARRAY_GET_LENGTH_FUNC_NAME, ProgramContext, ScopeKind, Type, VariableKind, Wasm}, wat};
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
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let (index_var_name, item_var_name) = match &self.iterator {
            ForIterator::Item(item_name) => (Identifier::unique("index", self), item_name.clone()),
            ForIterator::IndexAndItem(index_and_item) => (index_and_item.index_name.clone(), index_and_item.item_name.clone()),
        };
        let return_found = context.return_found;

        context.ckeck_var_unicity(&index_var_name);
        context.ckeck_var_unicity(&item_var_name);

        context.push_scope(ScopeKind::Loop);

        let mut ok = true;
        let mut content = vec![];
        let mut variables = vec![];

        let range_start_wasm_opt = self.range_start.process(context);
        let range_end_wasm_opt = self.range_end.as_ref().and_then(|expr| expr.process(context));

        if let Some(range_end) = &self.range_end {
            if let (Some(range_start_wasm), Some(range_end_wasm)) = (range_start_wasm_opt, range_end_wasm_opt) {
                if !range_start_wasm.ty.is_integer() {
                    context.error(&self.range_start, format!("range start: expected `{}`, got `{}`", Type::Integer, &range_start_wasm.ty));
                    ok = false;
                }

                if !range_end_wasm.ty.is_integer() {
                    context.error(range_end, format!("range end: expected `{}`, got `{}`", Type::Integer, &range_end_wasm.ty));
                    ok = false;
                }

                let range_end_var_name = Identifier::unique("range_end", self);

                let index_var_info = context.push_var(&index_var_name, &Type::Integer, VariableKind::Local);
                let item_var_info = context.push_var(&item_var_name, &Type::Integer, VariableKind::Local);
                let range_end_var_info = context.push_var(&range_end_var_name, &Type::Integer, VariableKind::Local);

                if let Some(block_wasm) = self.statements.process(context) {
                    content.extend(range_start_wasm.wat);
                    content.extend(range_end_wasm.wat);
                    content.extend(vec![
                        range_end_var_info.set_from_stack(),
                        item_var_info.set_from_stack(),
                        Wat::const_i32(-1),
                        index_var_info.set_from_stack(),
                        Wat::increment_local_i32(item_var_info.get_wasm_name(), -1),
                        wat!["block",
                            wat!["loop",
                                Wat::increment_local_i32(index_var_info.get_wasm_name(), 1),
                                Wat::increment_local_i32(item_var_info.get_wasm_name(), 1),
                                wat!["i32.lt_s", item_var_info.get_to_stack(), range_end_var_info.get_to_stack()],
                                wat!["br_if", 1, wat!["i32.eqz"]],
                                block_wasm.wat,
                                wat!["br", 0]
                            ]
                        ]
                    ]);

                    variables.extend(block_wasm.variables);
                } else {
                    ok = false;
                }

                variables.extend(vec![index_var_info, item_var_info, range_end_var_info]);
            }
        } else if let Some(array_wasm) = range_start_wasm_opt {
            if !array_wasm.ty.is_array() {
                if !array_wasm.ty.is_void() {
                    context.error(&self.range_start, format!("iterable: expected `{}`, got `{}`", Type::array(Type::Any(0)), &array_wasm.ty));
                }
                ok = false;
            }

            let array_var_name = Identifier::unique("array", self);
            let array_len_var_name = Identifier::unique("array_len", self);

            let array_var_info = context.push_var(&array_var_name, &Type::int_pointer(), VariableKind::Local);
            let array_len_var_info = context.push_var(&array_len_var_name, &Type::Integer, VariableKind::Local);
            let index_var_info = context.push_var(&index_var_name, &Type::Integer, VariableKind::Local);
            let item_var_info = context.push_var(&item_var_name, array_wasm.ty.get_item_type(), VariableKind::Local);

            let ptr_get_func_name = item_var_info.ty.pointer_get_function_name();

            if let Some(block_wasm) = self.statements.process(context) {
                content.extend(array_wasm.wat);
                content.extend(vec![
                    array_var_info.set_from_stack(),
                    Wat::const_i32(-1),
                    index_var_info.set_from_stack(),
                    Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![array_var_info.get_to_stack()]),
                    array_len_var_info.set_from_stack(),
                    Wat::call(ARRAY_GET_BODY_FUNC_NAME, vec![array_var_info.get_to_stack()]),
                    array_var_info.set_from_stack(),
                    wat!["block",
                        wat!["loop",
                            Wat::increment_local_i32(index_var_info.get_wasm_name(), 1),
                            wat!["i32.lt_s", index_var_info.get_to_stack(), array_len_var_info.get_to_stack()],
                            wat!["br_if", 1, wat!["i32.eqz"]],
                            Wat::set_local(item_var_info.get_wasm_name(), Wat::call(ptr_get_func_name, vec![array_var_info.get_to_stack(), index_var_info.get_to_stack()])),
                            block_wasm.wat,
                            wat!["br", 0]
                        ]
                    ]
                ]);

                variables.extend(block_wasm.variables);
            } else {
                ok = false;
            }

            variables.extend(vec![array_var_info, array_len_var_info, index_var_info, item_var_info]);
        } else {
            self.statements.process(context);
            ok = false;
        }

        context.pop_scope();
        context.return_found = return_found;

        match ok {
            true => Some(Wasm::new(Type::Void, content, variables)),
            false => None
        }
    }
}