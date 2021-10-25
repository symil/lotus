use parsable::parsable;
use crate::{program::{BuiltinType, NAN_WASM, ProgramContext, VI, Vasm}, wat};

#[parsable(name="float")]
pub struct FloatLiteral {
    #[parsable(regex = r"((-|\+)?\d+(\.\d*)?f)|nan")]
    pub value: String,
}

impl FloatLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let instruction = match self.value.as_str() {
            "nan" => VI::Raw(wat!["f32.const", NAN_WASM]),
            _ => VI::float(self.value[0..self.value.len() - 1].parse().unwrap())
        };

        Some(Vasm::new(context.get_builtin_type(BuiltinType::Float, vec![]), vec![], vec![instruction]))
    }
}