use pxp_parser::{node::Node, parser::ast::{StaticMethodCallExpression, Expression, identifiers::{Identifier, SimpleIdentifier}}, downcast::downcast};

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

use super::Rule;

#[derive(Debug)]
pub struct ValidStaticCallRule;

impl Rule for ValidStaticCallRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<StaticMethodCallExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let static_method_call = downcast::<StaticMethodCallExpression>(node).unwrap();

        let method_name = match static_method_call.method {
            Identifier::SimpleIdentifier(SimpleIdentifier { value: ref method_name, .. }) => method_name,
            _ => return,
        };

        let class_name = match static_method_call.target.as_ref() {
            Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { value: class_name, .. })) => class_name,
            _ => return,
        };

        let definition = definitions.get_class(class_name, context);

        if definition.is_none() {
            messages.add(format!(
                "Call to static method {}() on an unknown class {}",
                method_name,
                class_name
            ));
            return;
        }

        let definition = definition.unwrap();
        let method = definition.get_method(method_name);

        if method.is_none() {
            messages.add(format!(
                "Call to undefined static method {}::{}()",
                class_name,
                method_name
            ));

            return;
        }

        let method = method.unwrap();

        if !method.is_static() {
            messages.add(format!(
                "Static call to instance method {}::{}()",
                class_name,
                method_name
            ));

            return;
        }

        if method.is_abstract() {
            messages.add(format!(
                "Cannot call abstract static method {}::{}()",
                class_name,
                method_name
            ));

            return;
        }

        // TODO: Ensure method is:
        // 1. Public, or
        // 2. Protected and called within an allowed context
        // 3. Private and called within an allowed context
    }
}