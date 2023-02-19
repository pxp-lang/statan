use pxp_parser::{node::Node, downcast::downcast, parser::ast::{operators::AssignmentOperationExpression, Expression, variables::{Variable, SimpleVariable}}};

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}, shared::types::Type};

use super::Rule;

#[derive(Debug)]
pub struct ValidAssignmentRule;

impl Rule for ValidAssignmentRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<AssignmentOperationExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let assignment_operation_expression = downcast::<AssignmentOperationExpression>(node).unwrap();

        // 1. Check that we're doing a plain assignment.
        match assignment_operation_expression {
            AssignmentOperationExpression::Assign { .. } => {},
            _ => return,
        }

        // 2. Check that the left hand side is a plain variable.
        //    TODO: Add support for assigning to arrays and objects.
        let variable_name = match assignment_operation_expression.left() {
            Expression::Variable(Variable::SimpleVariable(SimpleVariable { name, .. })) => name,
            _ => return,
        };

        // 3. Get the type of the right-hand side.
        let value_type = context.get_type(assignment_operation_expression.right(), definitions);

        // 4. If the type of the right-hand side if `void` (null), we should warn.
        if value_type == Type::Void {
            messages.add(format!(
                "Assignment of void to variable {}",
                variable_name,
            ));
        }

        // 4. Enter the new variable type into the context.
        context.set_variable(variable_name.clone(), value_type);

        // 5. Assert the variable is in the context (it should be).
        debug_assert!(context.has_variable(variable_name));
    }
}