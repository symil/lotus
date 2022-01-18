use parsable::{ItemLocation, parsable};
use crate::{program::{BuiltinType, IS_NONE_METHOD_NAME, ProgramContext, Type, Vasm}, wat};
use super::{ParsedBinaryOperator, ParsedType, Identifier, ParsedOperand};

#[parsable]
pub struct ParsedBinaryOperation {
    pub first: ParsedOperand,
    pub others: Vec<(ParsedBinaryOperator, ParsedOperand)>
}

impl ParsedBinaryOperation {
    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        self.first.collect_instancied_type_names(list, context);
        
        for (_, operand) in &self.others {
            operand.collect_instancied_type_names(list, context);
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self.others.is_empty() {
            true => self.first.process(type_hint, context),
            false => OperationTree::from_operation(self).process(None, context),
        }
    }
}

#[derive(Debug)]
enum OperationTree<'a> {
    Operation(Box<OperationTree<'a>>, ParsedBinaryOperator, Box<OperationTree<'a>>),
    Value(&'a ParsedOperand)
}

impl<'a> OperationTree<'a> {
    fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            OperationTree::Operation(left, operator, right) => {
                let left_vasm_result = left.process(None, context);
                let right_type_hint = left_vasm_result.as_ref().and_then(|vasm| operator.get_type_hint(&vasm.ty, context));
                let right_vasm_result = right.process(right_type_hint.as_ref().map(|ty| ty.as_ref()), context);

                match (left_vasm_result, right_vasm_result) {
                    (Some(left_vasm), Some(right_vasm)) => operator.process(left_vasm, right_vasm, right.get_location(), context),
                    _ => None
                }
            },
            OperationTree::Value(operand) => operand.process(type_hint, context),
        }
    }

    fn from_operation(operation: &'a ParsedBinaryOperation) -> Self {
        let mut list : Vec<(ParsedBinaryOperator, &'a ParsedOperand, usize)> = operation.others.iter().enumerate().map(|(i, (operator, operand))| {
            let priority = operator.get_priority() * 256 + i;

            (operator.clone(), operand, priority)
        }).collect();

        list.insert(0, (ParsedBinaryOperator::default(), &operation.first, usize::MAX));

        Self::from_list(&mut list)
    }

    fn from_list(operands: &mut [(ParsedBinaryOperator, &'a ParsedOperand, usize)]) -> Self {
        if operands.len() == 1 {
            Self::Value(&operands[0].1)
        } else {
            let mut max_priority = 0;
            let mut index = 0;

            for (i, (_, _, priority)) in operands.iter().enumerate() {
                if *priority > max_priority && *priority != usize::MAX {
                    max_priority = *priority;
                    index = i
                }
            }

            let (left, mut right) = operands.split_at_mut(index);

            right[0].2 = usize::MAX;

            Self::Operation(
                Box::new(Self::from_list(left)),
                right[0].0.clone(),
                Box::new(Self::from_list(right))
            )
        }
    }

    fn get_location(&self) -> &'a ItemLocation {
        match self {
            OperationTree::Operation(left, _, _) => left.get_location(),
            OperationTree::Value(operand) => operand,
        }
    }
}