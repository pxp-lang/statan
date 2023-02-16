use pxp_parser::{node::Node, downcast::downcast, parser::ast::{FunctionCallExpression, identifiers::Identifier, Expression, arguments::Argument}, lexer::byte_string::ByteString};

use crate::{analyser::{context::Context, messages::MessageCollector}, definitions::collection::DefinitionCollection};

use super::Rule;

#[derive(Debug)]
pub struct DumpTypeRule;

impl Rule for DumpTypeRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<FunctionCallExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let function_call_expression = downcast::<FunctionCallExpression>(node).unwrap();

        let function_name = match function_call_expression.target.as_ref() {
            Expression::Identifier(Identifier::SimpleIdentifier(identifier)) => context.resolve_name(&identifier.value),
            _ => return,
        };

        if function_name != ByteString::from(b"\\Statan\\dumpType") {
            return;
        }

        let argument = match function_call_expression.arguments.arguments.first() {
            Some(Argument::Positional(argument)) => argument,
            Some(Argument::Named(_)) => {
                messages.add("dumpType() does not support named arguments");
                return;
            },
            None => {
                messages.add("dumpType() requires an argument");
                return;
            },
        };

        let ty = context.get_type(&argument.value, definitions);

        messages.add(format!("Dumped type: {:?}", ty));
    }
}