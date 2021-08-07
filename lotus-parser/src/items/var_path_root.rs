use parsable::{DataLocation, parsable};

use crate::{generation::{NULL_ADDR, Wat}, program::{AccessType, ProgramContext, Type, Wasm}};

use super::{ArrayLiteral, BooleanLiteral, FloatLiteral, IntegerLiteral, NullLiteral, ObjectLiteral, RootVarRef, StringLiteral, VarRef};

#[parsable]
pub enum VarPathRoot {
    NullLiteral(NullLiteral),
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    Variable(RootVarRef)
}

impl VarPathRoot {
    fn is_literal(&self) -> bool {
        match self {
            VarPathRoot::Variable(_) => false,
            _ => true
        }
    }

    fn get_location(&self) -> &DataLocation {
        match self {
            VarPathRoot::NullLiteral(x) => &x.location,
            VarPathRoot::BooleanLiteral(x) => &x.location,
            VarPathRoot::FloatLiteral(x) => &x.location,
            VarPathRoot::IntegerLiteral(x) => &x.location,
            VarPathRoot::StringLiteral(x) => &x.location,
            VarPathRoot::ArrayLiteral(x) => &x.location,
            VarPathRoot::ObjectLiteral(x) => &x.location,
            VarPathRoot::Variable(x) => &x.location,
        }
    }

    pub fn process(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        if access_type == AccessType::Set && self.is_literal() {
            context.error(self.get_location(), format!("cannot assign value to a literal"));

            return None;
        }

        match self {
            VarPathRoot::NullLiteral(null_literal) => null_literal.process(context),
            VarPathRoot::BooleanLiteral(boolean_literal) => boolean_literal.process(context),
            VarPathRoot::FloatLiteral(float_literal) => float_literal.process(context),
            VarPathRoot::IntegerLiteral(integer_literal) => integer_literal.process(context),
            VarPathRoot::StringLiteral(string_literal) => string_literal.process(context),
            VarPathRoot::ArrayLiteral(array_literal) => array_literal.process(context),
            VarPathRoot::ObjectLiteral(object_literal) => object_literal.process(context),
            VarPathRoot::Variable(_) => todo!(),
        }
    }
}