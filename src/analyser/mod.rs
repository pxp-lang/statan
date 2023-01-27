use pxp_parser::{parse, traverser::Visitor, node::Node, parser::ast::{UseStatement, Use, GroupUseStatement, namespaces::{UnbracedNamespace, BracedNamespace}, identifiers::SimpleIdentifier}, lexer::byte_string::ByteString, downcast::downcast};

use crate::{definitions::collection::DefinitionCollection, rules::Rule};

use self::{messages::MessageCollector, context::Context};

pub mod messages;
pub mod context;

#[derive(Debug)]
pub struct Analyser {
    rules: Vec<Box<dyn Rule>>,
    definitions: DefinitionCollection,
    message_collector: MessageCollector,
    context_stack: Vec<Context>,
}

impl Analyser {
    pub fn new(definitions: DefinitionCollection) -> Self {
        Self {
            rules: Vec::new(),
            definitions,
            message_collector: MessageCollector::default(),
            context_stack: Vec::new(),
        }
    }

    pub fn analyse(&mut self, file: String, contents: &[u8]) -> MessageCollector {
        self.message_collector = MessageCollector::new(file);

        let parse_result = parse(contents);
        if let Err(error) = parse_result {
            self.message_collector.add(error.to_string());
            return self.message_collector.clone();
        }

        let mut ast = parse_result.unwrap();

        self.context_stack.push(Context::new());
        self.visit_node(&mut ast).unwrap();

        return self.message_collector.clone();
    }

    pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }
}

impl Visitor<()> for Analyser {
    fn visit(&mut self, node: &mut dyn Node) -> Result<(), ()> {
        let mut context = self.context_stack.last_mut().unwrap();

        if let Some(BracedNamespace { name: Some(SimpleIdentifier { value, .. }), .. }) = downcast::<BracedNamespace>(node) {
            let mut namespace = ByteString::from(b"\\");
            namespace.extend(&value.bytes);
            context.set_namespace(namespace);
        }

        if let Some(UnbracedNamespace { name: SimpleIdentifier { value, .. }, .. }) = downcast::<UnbracedNamespace>(node) {
            let mut namespace = ByteString::from(b"\\");
            namespace.extend(&value.bytes);
            context.set_namespace(namespace);
        }

        if let Some(GroupUseStatement { prefix, uses, .. }) = downcast::<GroupUseStatement>(node) {
            for Use { name, .. } in uses {
                let mut prefixed_name = prefix.value.clone();
                prefixed_name.extend(b"\\");
                prefixed_name.extend(&name.value.bytes);

                context.add_import(prefixed_name);
            }
        }

        if let Some(UseStatement { uses, .. }) = downcast::<UseStatement>(node) {
            for Use { name, .. } in uses {
                let mut qualified_name = ByteString::from(b"\\");
                qualified_name.extend(&name.value.bytes);
                context.add_import(qualified_name);
            }
        }

        for rule in &mut self.rules {
            if rule.should_run(node) {
                rule.run(node, &self.definitions, &mut self.message_collector, &mut context);
            }
        }

        Ok(())
    }
}