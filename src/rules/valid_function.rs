use pxp_parser::{node::Node, downcast::downcast, parser::ast::{FunctionCallExpression, Expression, identifiers::{Identifier, SimpleIdentifier}}, lexer::byte_string::ByteString};

use crate::{rules::Rule, definitions::collection::DefinitionCollection, analyser::{messages::{self, MessageCollector}, context::Context}};

#[derive(Debug)]
pub struct ValidFunctionRule;

impl Rule for ValidFunctionRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<FunctionCallExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let function_call_expression = downcast::<FunctionCallExpression>(node).unwrap();

        match function_call_expression.target.as_ref() {
            Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { value: function_name, .. })) => { 
                if definitions.get_function(&context.resolve_name(function_name)).is_none() {
                    messages.add(format!("Function `{}` not found", function_name));
                }
            },
            _ => return,
        }
    }
}