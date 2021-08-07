use parsable::{DataLocation, parsable};
use crate::{generation::{Wat, ToWat, ToWatVec}, merge, program::{ProgramContext, Type, Wasm}};
use super::{BinaryOperator, Operand, FullType};

#[parsable]
pub struct BinaryOperation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>
}

impl BinaryOperation {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let operation_tree = OperationTree::from_operation(self);

        operation_tree.process(context)
    }
}

enum OperationTree<'a> {
    Operation(Box<OperationTree<'a>>, BinaryOperator, Box<OperationTree<'a>>),
    Value(&'a Operand)
}

impl<'a> OperationTree<'a> {
    fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            OperationTree::Operation(left, operator, right) => {
                let left_wasm_result = left.process(context);
                let right_wasm_result = right.process(context);

                match (left_wasm_result, right_wasm_result) {
                    (Some(left_wasm), Some(right_wasm)) => {
                        let mut result = None;
                        let left_result = operator.process(&left_wasm.ty, context);
                        let right_result = operator.process(&right_wasm.ty, context);
                        let same_type = left_wasm.ty.is_compatible(&right_wasm.ty, context);

                        if !left_result.is_none() {
                            context.error(left.get_location(), format!("operator `{}`: invalid left operand type `{}`", operator, &left_wasm.ty));
                        }

                        if !right_result.is_none() {
                            context.error(right.get_location(), format!("operator `{}`: invalid right operand type `{}`", operator, &right_wasm.ty));
                        }

                        if left_result.is_some() && right_result.is_some() && !same_type {
                            context.error(left.get_location(), format!("operator `{}`: operand types must match (got `{}` and `{}`)", operator, &left_wasm.ty, &right_wasm.ty));
                        } else {
                            if let Some(operator_wasm) = left_result {
                                if let Some(_) = right_result {
                                    result = Some(Wasm::typed(operator_wasm.ty, merge![left_wasm.wat, right_wasm.wat, operator_wasm.wat]));
                                }
                            }
                        }

                        result
                    },
                    _ => None
                }
            },
            OperationTree::Value(operand) => operand.process(context),
        }
    }

    fn from_operation(operation: &'a BinaryOperation) -> Self {
        let mut list : Vec<(BinaryOperator, &'a Operand, usize)> = operation.others.iter().enumerate().map(|(i, (operator, operand))| {
            let priority = operator.get_priority() * 256 + i;

            (operator.clone(), operand, priority)
        }).collect();

        list.insert(0, (BinaryOperator::Plus, &operation.first, usize::MAX));

        Self::from_list(&mut list)
    }

    fn from_list(operands: &mut [(BinaryOperator, &'a Operand, usize)]) -> Self {
        if operands.len() == 1 {
            Self::Value(&operands[0].1)
        } else {
            let mut max_priority = usize::MAX;
            let mut index = 0;

            for (i, (_, _, priority)) in operands.iter().enumerate() {
                if *priority > max_priority {
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