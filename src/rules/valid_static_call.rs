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

        // CRITERIA:
        // - Only check static calls to known classes (identifiers)
        //      - Also check static calls to self, static & parent
        // - Only check static calls to known methods (identifiers)
        // CHECKS:
        // - Check if class exists
        // - Check if method exists
        //      - Check if method is static
        //      - Check if method is not abstract
        //      - Check if method is public, or protected and called within an allowed context, or private and called within an allowed context
        // - If method doesn't exist, check if class has a __callStatic() method

        todo!()
    }
}