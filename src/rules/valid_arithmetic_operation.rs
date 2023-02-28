use pxp_parser::{node::Node, downcast::downcast, parser::ast::{operators::ArithmeticOperationExpression, Expression}};

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}, shared::types::Type};

use super::Rule;

#[derive(Debug)]
pub struct ValidArithmeticOperationRule;

impl Rule for ValidArithmeticOperationRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<Expression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let expression = downcast::<Expression>(node).unwrap();

        if let Expression::ArithmeticOperation(operation) = expression {
            match context.get_type(expression, definitions) {
                Type::Error => match operation {
                    ArithmeticOperationExpression::Addition { left, plus, right } => messages.error(
                        format!("Arithmetic operation + between {} and {} is invalid", context.get_type(left, definitions), context.get_type(right, definitions)),
                        plus.line,
                    ),
                    ArithmeticOperationExpression::Subtraction { left, minus, right } => messages.error(
                        format!("Arithmetic operation - between {} and {} is invalid", context.get_type(left, definitions), context.get_type(right, definitions)),
                        minus.line,
                    ),
                    ArithmeticOperationExpression::Multiplication { left, asterisk, right } => messages.error(
                        format!("Arithmetic operation * between {} and {} is invalid", context.get_type(left, definitions), context.get_type(right, definitions)),
                        asterisk.line,
                    ),
                    ArithmeticOperationExpression::Division { left, slash, right } => messages.error(
                        format!("Arithmetic operation / between {} and {} is invalid", context.get_type(left, definitions), context.get_type(right, definitions)),
                        slash.line,
                    ),
                    ArithmeticOperationExpression::Modulo { left, percent, right } => messages.error(
                        format!("Arithmetic operation % between {} and {} is invalid", context.get_type(left, definitions), context.get_type(right, definitions)),
                        percent.line,
                    ),
                    ArithmeticOperationExpression::Exponentiation { left, pow, right } => messages.error(
                        format!("Arithmetic operation ** between {} and {} is invalid", context.get_type(left, definitions), context.get_type(right, definitions)),
                        pow.line,
                    ),
                    ArithmeticOperationExpression::Negative { minus, right } => messages.error(
                        format!("Arithmetic operation -{} is invalid", context.get_type(right, definitions)),
                        minus.line,
                    ),
                    ArithmeticOperationExpression::Positive { plus, right } => messages.error(
                        format!("Arithmetic operation +{} is invalid", context.get_type(right, definitions)),
                        plus.line,
                    ),
                    ArithmeticOperationExpression::PreIncrement { increment, right } => messages.error(
                        format!("Arithmetic operation ++{} is invalid", context.get_type(right, definitions)),
                        increment.line,
                    ),
                    ArithmeticOperationExpression::PostIncrement { left, increment } => messages.error(
                        format!("Arithmetic operation {}++ is invalid", context.get_type(left, definitions)),
                        increment.line,
                    ),
                    ArithmeticOperationExpression::PreDecrement { decrement, right } => messages.error(
                        format!("Arithmetic operation --{} is invalid", context.get_type(right, definitions)),
                        decrement.line,
                    ),
                    ArithmeticOperationExpression::PostDecrement { left, decrement } => messages.error(
                        format!("Arithmetic operation {}-- is invalid", context.get_type(left, definitions)),
                        decrement.line,
                    ),
                },
                _ => return,
            }
        }
    }
}