use crate::{items::{BinaryOperator, Operand, Operation}, program::{BuiltinType, ExpressionType}};

pub enum OperationTree<'a> {
    Operation(Box<OperationTree<'a>>, BinaryOperator, Box<OperationTree<'a>>),
    Value(&'a Operand)
}

impl<'a> OperationTree<'a> {
    pub fn from_operation(operation: &'a Operation) -> Self {
        let mut list : Vec<(BinaryOperator, &'a Operand, usize)> = operation.others.iter().enumerate().map(|(i, (operator, operand))| {
            let priority = get_operator_priority(operator) * 256 + i;

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

    pub fn get_leftmost(&self) -> &'a Operand {
        match self {
            OperationTree::Operation(left, _, _) => left.get_leftmost(),
            OperationTree::Value(operand) => operand,
        }
    }
}

fn get_operator_priority(operator: &BinaryOperator) -> usize {
    match operator {
        BinaryOperator::Mult | BinaryOperator::Div | BinaryOperator::Mod => 0,
        BinaryOperator::Plus | BinaryOperator::Minus => 1,
        BinaryOperator::Range => 2,
        BinaryOperator::Eq | BinaryOperator::Neq | BinaryOperator::Gte | BinaryOperator::Gt | BinaryOperator::Lte | BinaryOperator::Lt => 3,
        BinaryOperator::And => 4,
        BinaryOperator::Or => 5,
    }
}

pub fn get_binary_operator_input_types(operator: &BinaryOperator) -> Vec<ExpressionType> {
    match operator {
        BinaryOperator::Plus => vec![
            ExpressionType::builtin(BuiltinType::Integer),
            ExpressionType::builtin(BuiltinType::String),
            ExpressionType::array(ExpressionType::Anonymous(0)),
        ],
        BinaryOperator::Minus | BinaryOperator::Mult | BinaryOperator::Div | BinaryOperator::Mod => vec![ExpressionType::builtin(BuiltinType::Integer)],
        BinaryOperator::Gte | BinaryOperator::Gt | BinaryOperator::Lte | BinaryOperator::Lt => vec![ExpressionType::builtin(BuiltinType::Integer)],
        BinaryOperator::And | BinaryOperator::Or => vec![ExpressionType::builtin(BuiltinType::Boolean)],
        BinaryOperator::Eq | BinaryOperator::Neq => vec![ExpressionType::Anonymous(0)],
        BinaryOperator::Range => vec![ExpressionType::builtin(BuiltinType::Integer)],
    }
}

pub fn get_binary_operator_output_type(operator: &BinaryOperator, operand_type: &ExpressionType) -> ExpressionType {
    match operator {
        BinaryOperator::Plus | BinaryOperator::Minus | BinaryOperator::Mult | BinaryOperator::Div | BinaryOperator::Mod => operand_type.clone(),
        BinaryOperator::And | BinaryOperator::Or => operand_type.clone(),
        BinaryOperator::Eq | BinaryOperator::Neq => ExpressionType::builtin(BuiltinType::Boolean),
        BinaryOperator::Gte | BinaryOperator::Gt | BinaryOperator::Lte | BinaryOperator::Lt => ExpressionType::builtin(BuiltinType::Boolean),
        BinaryOperator::Range => ExpressionType::array(ExpressionType::builtin(BuiltinType::Integer))
    }
}