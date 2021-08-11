use parsable::parsable;
use crate::{generation::{ARRAY_LENGTH_FUNC_NAME, ToWat, ToWatVec, Wat}, program::{ProgramContext, Type, Wasm}, wat};
use super::{Expression, Identifier, Statement, StatementList};

#[parsable]
pub struct ForBlock {
    #[parsable(prefix="for")]
    pub iterator: ForIterator,
    #[parsable(prefix="in")]
    pub array_expression: Expression,
    #[parsable(brackets="{}")]
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
        let mut result = None;
        let array_var_name = Identifier::new(format!("__array_{}_{}", self.location.start, self.location.end));
        let array_len_var_name = Identifier::new(format!("__array_len_{}_{}", self.location.start, self.location.end));
        let default_index_identifier = Identifier::new(format!("__i_{}_{}", self.location.start, self.location.end));
        let (index_var_name, item_var_name) = match &self.iterator {
            ForIterator::Item(item_name) => (&default_index_identifier, item_name),
            ForIterator::IndexAndItem(index_and_item) => (&index_and_item.index_name, &index_and_item.item_name),
        };
        let return_found = context.return_found;

        if context.var_exists(index_var_name) {
            context.error(index_var_name, format!("duplicate variable declaration: `{}`", index_var_name));
        }

        if context.var_exists(item_var_name) {
            context.error(item_var_name, format!("duplicate variable declaration: `{}`", item_var_name));
        }

        context.function_depth += 2;

        if let (Some(array_wasm), Some(block_wasm)) = (self.array_expression.process(context), self.statements.process(context)) {            
            let mut content = vec![];

            context.push_local_var(&array_var_name, &Type::Pointer);
            context.push_local_var(&array_len_var_name, &Type::Integer);
            context.push_local_var(&index_var_name, &Type::Integer);
            context.push_local_var(&item_var_name, array_wasm.ty.get_item_type());

            content.extend(array_wasm.wat);
            content.push(Wat::set_local_from_stack(array_var_name.as_str()));
            content.push(Wat::set_local(index_var_name.as_str(), Wat::const_i32(0)));
            content.push(Wat::set_local(array_len_var_name.as_str(), Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![Wat::get_local(array_len_var_name.as_str())])));
            content.push(wat!["block",
                wat!["loop",
                    wat!["i32.lt", Wat::get_local(index_var_name.as_str()), Wat::get_local(array_len_var_name.as_str())],
                    wat!["br_if", 1, wat!["i32.eqz"]],
                    block_wasm.wat,
                    Wat::increment_local_i32(index_var_name.as_str(), 1),
                    wat!["br", 0]
                ]
            ]);

            result = Some(Wasm::untyped(content));
        }

        context.return_found = return_found;
        context.function_depth -= 2;

        result
    }
}