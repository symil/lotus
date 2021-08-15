use parsable::parsable;
use crate::{generation::{ARRAY_LENGTH_FUNC_NAME, ToWat, ToWatVec, Wat}, program::{ProgramContext, Type, VariableScope, Wasm}, wat};
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

        if context.var_exists(&index_var_name) {
            context.error(&index_var_name, format!("duplicate variable declaration: `{}`", &index_var_name));
        }

        if context.var_exists(&item_var_name) {
            context.error(&item_var_name, format!("duplicate variable declaration: `{}`", &item_var_name));
        }

        context.function_depth += 2;

        let mut ok = true;
        let mut content = vec![];

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

                context.push_var(&index_var_name, &Type::Integer, VariableScope::Local);
                context.push_var(&item_var_name, &Type::Integer, VariableScope::Local);
                context.push_var(&range_end_var_name, &Type::Integer, VariableScope::Local);

                if let Some(block_wasm) = self.statements.process(context) {
                    content.extend(range_start_wasm.wat);
                    content.extend(range_end_wasm.wat);
                    content.extend(vec![
                        Wat::set_local_from_stack(range_end_var_name.as_str()),
                        Wat::set_local_from_stack(item_var_name.as_str()),
                        Wat::set_local(index_var_name.as_str(), Wat::const_i32(0)),
                        wat!["block",
                            wat!["loop",
                                wat!["i32.lt_s", Wat::get_local(item_var_name.as_str()), Wat::get_local(range_end_var_name.as_str())],
                                wat!["br_if", 1, wat!["i32.eqz"]],
                                block_wasm.wat,
                                Wat::increment_local_i32(item_var_name.as_str(), 1),
                                Wat::increment_local_i32(index_var_name.as_str(), 1),
                                wat!["br", 0]
                            ]
                        ]
                    ]);
                } else {
                    ok = false;
                }
            }
        } else if let Some(array_wasm) = range_start_wasm_opt {
            if !array_wasm.ty.is_array() {
                context.error(&self.range_start, format!("iterable: expected `{}`, got `{}`", Type::array(Type::Any(0)), &array_wasm.ty));
                ok = false;
            }

            let array_var_name = Identifier::new_unique("array", self);
            let array_len_var_name = Identifier::new_unique("array_len", self);

            context.push_var(&array_var_name, &Type::int_pointer(), VariableScope::Local);
            context.push_var(&array_len_var_name, &Type::Integer, VariableScope::Local);
            context.push_var(&index_var_name, &Type::Integer, VariableScope::Local);
            context.push_var(&item_var_name, array_wasm.ty.get_item_type(), VariableScope::Local);

            if let Some(block_wasm) = self.statements.process(context) {
                content.extend(array_wasm.wat);
                content.extend(vec![
                    Wat::set_local_from_stack(array_var_name.as_str()),
                    Wat::set_local(index_var_name.as_str(), Wat::const_i32(0)),
                    Wat::set_local(array_len_var_name.as_str(), Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![Wat::get_local(array_len_var_name.as_str())])),
                    wat!["block",
                        wat!["loop",
                            wat!["i32.lt", Wat::get_local(index_var_name.as_str()), Wat::get_local(array_len_var_name.as_str())],
                            wat!["br_if", 1, wat!["i32.eqz"]],
                            block_wasm.wat,
                            Wat::increment_local_i32(index_var_name.as_str(), 1),
                            wat!["br", 0]
                        ]
                    ]
                ]);
            } else {
                ok = false;
            }
        } else {
            self.statements.process(context);
            ok = false;
        }

        context.return_found = return_found;
        context.function_depth -= 2;

        match ok {
            true => Some(Wasm::untyped(content)),
            false => None
        }
    }
}