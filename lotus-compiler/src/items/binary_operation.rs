use parsable::{DataLocation, parsable};
use crate::{program::{BuiltinType, IS_NONE_METHOD_NAME, ProgramContext, Type, VI, Vasm}, wat};
use super::{BinaryOperatorWrapper, ParsedType, Identifier, Operand};

#[parsable]
pub struct BinaryOperation {
    pub first: Operand,
    pub others: Vec<(BinaryOperatorWrapper, Operand)>
}

impl BinaryOperation {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
        self.first.collected_instancied_type_names(list, context);
        
        for (_, operand) in &self.others {
            operand.collected_instancied_type_names(list, context);
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self.others.is_empty() {
            true => self.first.process(type_hint, context),
            false => OperationTree::from_operation(self).process(context),
        }
    }
}

#[derive(Debug)]
enum OperationTree<'a> {
    Operation(Box<OperationTree<'a>>, BinaryOperatorWrapper, Box<OperationTree<'a>>),
    Value(&'a Operand)
}

impl<'a> OperationTree<'a> {
    fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            OperationTree::Operation(left, operator, right) => {
                let left_vasm_result = left.process(context);
                let right_vasm_result = right.process(context);

                match (left_vasm_result, right_vasm_result) {
                    (Some(left_vasm), Some(right_vasm)) => operator.process(left_vasm, right_vasm, right.get_location(), context),
                    _ => None
                }
            },
            OperationTree::Value(operand) => operand.process(None, context),
        }
    }

    fn from_operation(operation: &'a BinaryOperation) -> Self {
        let mut list : Vec<(BinaryOperatorWrapper, &'a Operand, usize)> = operation.others.iter().enumerate().map(|(i, (operator, operand))| {
            let priority = operator.get_priority() * 256 + i;

            (operator.clone(), operand, priority)
        }).collect();

        list.insert(0, (BinaryOperatorWrapper::default(), &operation.first, usize::MAX));

        Self::from_list(&mut list)
    }

    fn from_list(operands: &mut [(BinaryOperatorWrapper, &'a Operand, usize)]) -> Self {
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

    fn get_location(&self) -> &'a DataLocation {
        match self {
            OperationTree::Operation(left, _, _) => left.get_location(),
            OperationTree::Value(operand) => operand.get_location(),
        }
    }
}