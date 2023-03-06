use pxp_parser::{
    downcast::downcast,
    lexer::byte_string::ByteString,
    node::Node,
    parser::ast::{
        identifiers::{Identifier, SimpleIdentifier},
        Expression, NewExpression,
    },
};

use crate::{
    analyser::{context::Context, messages::MessageCollector},
    definitions::collection::DefinitionCollection,
    rules::Rule,
};

#[derive(Debug)]
pub struct ValidClassRule;

impl Rule for ValidClassRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<NewExpression>(node).is_some()
    }

    fn run(
        &mut self,
        node: &mut dyn Node,
        definitions: &DefinitionCollection,
        messages: &mut MessageCollector,
        context: &mut Context,
    ) {
        let new_expression = downcast::<NewExpression>(node).unwrap();

        let (name, span) = match new_expression.target.as_ref() {
            Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier {
                value: class_name,
                span,
            })) => (class_name, span),
            _ => return,
        };

        let definition = definitions.get_class(name, context);

        if definition.is_none() {
            // TODO: Add a check for execution inside of a `class_exists` call.
            messages.error(
                format!(
                    "Class `{}` (DBG: {}, {}) not found",
                    name,
                    context.resolve_name(name),
                    {
                        let mut global_name = ByteString::default();
                        global_name.extend(b"\\");
                        global_name.extend(&name.bytes);
                        global_name
                    }
                ),
                span.line,
            );

            return;
        }

        let definition = definition.unwrap();

        if definition.is_abstract() {
            messages.error(
                format!("Cannot instantiate abstract class `{name}`"),
                span.line,
            );
        }
    }
}
