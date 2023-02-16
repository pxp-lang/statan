use pxp_parser::{node::Node, downcast::downcast, parser::ast::{FunctionCallExpression, Expression, identifiers::{Identifier, SimpleIdentifier}}, lexer::byte_string::ByteString};

use crate::{rules::Rule, definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

#[derive(Debug)]
pub struct ValidFunctionRule;

impl Rule for ValidFunctionRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<FunctionCallExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let function_call_expression = downcast::<FunctionCallExpression>(node).unwrap();

        let name = match function_call_expression.target.as_ref() {
            Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { value: function_name, .. })) => function_name,
            _ => return,
        };

        if definitions.get_function(name, context).is_some() {
            return;
        }

        // TODO: Add a check for execution inside of a `function_exists` call.
        messages.add(format!("Function `{}` (DBG: {}, {}) not found", name, context.resolve_name(name), {
            let mut global_name = ByteString::default();
            global_name.extend(b"\\");
            global_name.extend(&name.bytes);
            global_name
        }));
    }
}