use pxp_parser::{
    downcast::downcast,
    node::Node,
    parser::ast::{classes::ClassMember, functions::AbstractMethod},
};

use crate::{
    analyser::{context::Context, messages::MessageCollector},
    definitions::collection::DefinitionCollection,
};

use super::Rule;

#[derive(Debug)]
pub struct AbstractMethodInNonAbstractClassRule;

impl Rule for AbstractMethodInNonAbstractClassRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        match downcast(node) {
            Some(ClassMember::AbstractMethod(_)) => true,
            _ => false,
        }
    }

    fn run(
        &mut self,
        node: &mut dyn Node,
        definitions: &DefinitionCollection,
        messages: &mut MessageCollector,
        context: &mut Context,
    ) {
        if !context.is_in_class() {
            unreachable!();
        }

        let current_class = context.classish_context();
        let current_class = definitions.get_class(current_class, context).unwrap();

        if current_class.is_abstract() {
            return;
        }

        let (method_definition, span) = match downcast::<ClassMember>(node).unwrap() {
            ClassMember::AbstractMethod(AbstractMethod { name, .. }) => (
                current_class
                    .get_method(&name.value, definitions, context)
                    .unwrap(),
                name.span,
            ),
            _ => unreachable!(),
        };

        if !method_definition.is_abstract() {
            return;
        }

        messages.error(
            format!(
                "Non-abstract class {} contains abstract method {}",
                current_class.name, method_definition.name
            ),
            span.line,
        );
    }
}
