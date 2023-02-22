use pxp_parser::{node::Node, downcast::downcast, parser::ast::{StaticMethodCallExpression, Expression, identifiers::{Identifier, SimpleIdentifier}}};

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

use super::Rule;

#[derive(Debug)]
pub struct CallPrivateThroughStaticRule;

impl Rule for CallPrivateThroughStaticRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<StaticMethodCallExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        if ! context.is_in_class() {
            return;
        }

        let static_method_call_expression = downcast::<StaticMethodCallExpression>(node).unwrap();

        if static_method_call_expression.target.as_ref() != &Expression::Static {
            return;
        }

        let (method_name, span) = match &static_method_call_expression.method {
            Identifier::SimpleIdentifier(SimpleIdentifier { span, value }) => (value, span),
            _ => return,
        };

        let current_class = context.classish_context();
        let current_class = definitions.get_class(current_class, context).unwrap();

        if current_class.is_final() {
            return;
        }

        let method_definition = current_class.get_method(&method_name, definitions, context);

        if method_definition.is_none() {
            return;
        }

        let method_definition = method_definition.unwrap();

        if !method_definition.is_private() {
            return;
        }

        messages.error(format!(
            "Unsafe call to private method {}::{}() on static::",
            current_class.name,
            method_name,
        ), span.line);
    }
}