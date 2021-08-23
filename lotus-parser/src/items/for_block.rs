use parsable::parsable;
use crate::{generation::{ARRAY_LENGTH_FUNC_NAME, ToWat, ToWatVec, Wat}, program::{ProgramContext, ScopeKind, Type, VariableKind, Wasm}, wat};
use super::{Expression, Identifier, Statement, StatementList};

#[parsable]
pub struct ForBlock {
    #[parsable(prefix="for")]
    pub iterator: ForIterator,
    #[parsable(prefix="in")]
    pub range_start: Expression,
    #[parsable(prefix="..")]
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
            ForIterator::Item(item_name) => (Identifier::new_unique("index", self), item_name.clone()),
            ForIterator::IndexAndItem(index_and_item) => (index_and_item.index_name.clone(), index_and_item.item_name.clone()),
        };
        let return_found = context.return_found;

        context.ckeck_var_unicity(&index_var_name);
        context.ckeck_var_unicity(&item_var_name);

        context.function_depth += 2;
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

                let range_end_var_name = Identifier::new_unique("range_end", self);

                let index_var_info = context.push_var(&index_var_name, &Type::Integer, VariableKind::Local);
                let item_var_info = context.push_var(&item_var_name, &Type::Integer, VariableKind::Local);
                let range_end_var_info = context.push_var(&range_end_var_name, &Type::Integer, VariableKind::Local);

                if let Some(block_wasm) = self.statements.process(context) {
                    content.extend(range_start_wasm.wat);
                    content.extend(range_end_wasm.wat);
                    content.extend(vec![
                        range_end_var_info.set_from_stack(),
                        item_var_info.set_from_stack(),
                        Wat::const_i32(0),
                        index_var_info.set_from_stack(),
                        wat!["block",
                            wat!["loop",
                                wat!["i32.lt_s", item_var_info.get_to_stack(), range_end_var_info.get_to_stack()],
                                wat!["br_if", 1, wat!["i32.eqz"]],
                                block_wasm.wat,
                                Wat::increment_local_i32(item_var_info.get_wasm_name(), 1),
                                Wat::increment_local_i32(index_var_info.get_wasm_name(), 1),
                                wat!["br", 0]
                            ]
                        ]
                    ]);
                } else {
                    ok = false;
                }

                variables.extend(vec![index_var_info, item_var_info, range_end_var_info]);
            }
        } else if let Some(array_wasm) = range_start_wasm_opt {
            if !array_wasm.ty.is_array() {
                context.error(&self.range_start, format!("iterable: expected `{}`, got `{}`", Type::array(Type::Any(0)), &array_wasm.ty));
                ok = false;
            }

            let array_var_name = Identifier::new_unique("array", self);
            let array_len_var_name = Identifier::new_unique("array_len", self);

            let array_var_info = context.push_var(&array_var_name, &Type::int_pointer(), VariableKind::Local);
            let array_len_var_info = context.push_var(&array_len_var_name, &Type::Integer, VariableKind::Local);
            let index_var_info = context.push_var(&index_var_name, &Type::Integer, VariableKind::Local);
            let item_var_info = context.push_var(&item_var_name, array_wasm.ty.get_item_type(), VariableKind::Local);

            if let Some(block_wasm) = self.statements.process(context) {
                content.extend(array_wasm.wat);
                content.extend(vec![
                    array_var_info.set_from_stack(),
                    Wat::const_i32(0),
                    index_var_info.set_from_stack(),
                    Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![array_var_info.get_to_stack()]),
                    array_len_var_info.set_from_stack(),
                    wat!["block",
                        wat!["loop",
                            wat!["i32.lt", index_var_info.get_to_stack(), array_len_var_info.get_to_stack()],
                            wat!["br_if", 1, wat!["i32.eqz"]],
                            block_wasm.wat,
                            Wat::increment_local_i32(index_var_info.get_wasm_name(), 1),
                            wat!["br", 0]
                        ]
                    ]
                ]);
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
        context.function_depth -= 2;

        match ok {
            true => Some(Wasm::untyped(content, variables)),
            false => None
        }
    }
}