use pxp_parser::{node::Node, parser::ast::{operators::AssignmentOperationExpression, Expression, FunctionCallExpression, identifiers::Identifier}, downcast::downcast};

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

use super::Rule;

#[derive(Debug)]
pub struct VoidAssignmentRule;

impl Rule for VoidAssignmentRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<AssignmentOperationExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let assignment_operation_expression = downcast::<AssignmentOperationExpression>(node).unwrap();

        let (left, right) = match assignment_operation_expression {
            AssignmentOperationExpression::Assign { left, right, .. } => (left, right),
            _ => return,
        };

        // TODO: Also support method calls, not just functions.
        if ! matches!(right.as_ref(), Expression::FunctionCall(_)) {
            return;
        }

        let function_name = match right.as_ref() {
            Expression::FunctionCall(FunctionCallExpression { target, .. }) => match target.as_ref() {
                Expression::Identifier(Identifier::SimpleIdentifier(identifier)) => identifier,
                _ => return,
            },
            _ => return,
        };

        // NOTE: Invalid function calls are handled by ValidFunctionRule.
        let function_definition = match definitions.get_function(&function_name.value, context) {
            Some(function_definition) => function_definition,
            None => return,
        };

        if function_definition.returns_void() {
            messages.add(format!(
                "Result of function {} (void) is used",
                function_name,
            ));
        }
    }
}