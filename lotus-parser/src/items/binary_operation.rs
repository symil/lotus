use parsable::{DataLocation, parsable};
use crate::{generation::{Wat, ToWat, ToWatVec}, program::{ProgramContext, TypeOld, IrFragment}, wat};
use super::{BinaryOperatorWrapper, Operand, FullType};

#[parsable]
pub struct BinaryOperation {
    pub first: Operand,
    pub others: Vec<(BinaryOperatorWrapper, Operand)>
}

impl BinaryOperation {
    pub fn has_side_effects(&self) -> bool {
        match self.first.has_side_effects() {
            true => true,
            false => self.others.iter().any(|(_, operand)| operand.has_side_effects())
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let operation_tree = OperationTree::from_operation(self);

        operation_tree.process(context)
    }
}

#[derive(Debug)]
enum OperationTree<'a> {
    Operation(Box<OperationTree<'a>>, BinaryOperatorWrapper, Box<OperationTree<'a>>),
    Value(&'a Operand)
}

impl<'a> OperationTree<'a> {
    fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        match self {
            OperationTree::Operation(left, operator, right) => {
                let left_wasm_result = left.process(context);
                let right_wasm_result = right.process(context);

                match (left_wasm_result, right_wasm_result) {
                    (Some(left_wasm), Some(right_wasm)) => {
                        let mut result = None;
                        let operator_wasm_opt = operator.process(&left_wasm.ty, &right_wasm.ty, context);

                        // TODO: if operator is `&&` or `||`, convert operands to booleans when possible

                        if let Some(operator_wasm) = operator_wasm_opt {
                            let wasm = if let Some(short_circuit_wasm) = operator.get_short_circuit_wasm(context) {
                                if right.has_side_effects() {
                                    let mut wasm = IrFragment::merge(operator_wasm.ty.clone(), vec![left_wasm, short_circuit_wasm, right_wasm, operator_wasm]);

                                    wasm.wat = vec![wat!["block", wat!["result", "i32"], wasm.wat]];

                                    wasm
                                } else {
                                    IrFragment::merge(operator_wasm.ty.clone(), vec![left_wasm, right_wasm, operator_wasm])
                                }
                            } else {
                                IrFragment::merge(operator_wasm.ty.clone(), vec![left_wasm, right_wasm, operator_wasm])
                            };

                            result = Some(wasm);
                        }

                        result
                    },
                    _ => None
                }
            },
            OperationTree::Value(operand) => operand.process(context),
        }
    }

    fn has_side_effects(&self) -> bool {
        match self {
            OperationTree::Operation(left, _, right) => left.has_side_effects() || right.has_side_effects(),
            OperationTree::Value(operand) => operand.has_side_effects() ,
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