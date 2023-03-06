use pxp_parser::{
    downcast::downcast,
    lexer::byte_string::ByteString,
    node::Node,
    parse,
    parser::ast::{
        classes::ClassStatement,
        functions::FunctionStatement,
        identifiers::SimpleIdentifier,
        namespaces::{BracedNamespace, UnbracedNamespace},
        GroupUseStatement, Use, UseStatement,
    },
    traverser::Visitor,
};

use crate::{definitions::collection::DefinitionCollection, rules::Rule, shared::types::Type};

use self::{context::Context, messages::MessageCollector};

pub mod context;
pub mod messages;

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
            self.message_collector.error(error.to_string(), 0);
            return self.message_collector.clone();
        }

        let mut ast = parse_result.unwrap();

        self.context_stack.push(Context::new());
        self.visit_node(&mut ast).unwrap();
        self.message_collector.clone()
    }

    pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }
}

impl Visitor<()> for Analyser {
    fn visit_node(&mut self, node: &mut dyn Node) -> Result<(), ()> {
        self.visit(node)?;

        let mut context = self.context_stack.last_mut().unwrap().clean();
        let mut did_push_context = false;

        if let Some(ClassStatement { name, .. }) = downcast(node) {
            context.set_classish_context(&name.value);
            self.context_stack.push(context);
            did_push_context = true;
        } else if let Some(FunctionStatement {
            name, parameters, ..
        }) = downcast(node)
        {
            context.set_function_context(&name.value);
            for parameter in parameters.iter() {
                context.set_variable(
                    parameter.name.name.clone(),
                    parameter
                        .data_type
                        .as_ref()
                        .map(|t| t.into())
                        .unwrap_or(Type::Mixed)
                        .into(),
                );
            }
            self.context_stack.push(context);
            did_push_context = true;
        }

        for child in node.children() {
            self.visit_node(child)?;
        }

        if did_push_context {
            self.context_stack.pop();
        }

        Ok(())
    }

    fn visit(&mut self, node: &mut dyn Node) -> Result<(), ()> {
        let context = self.context_stack.last_mut().unwrap();

        if let Some(BracedNamespace {
            name: Some(SimpleIdentifier { value, .. }),
            ..
        }) = downcast::<BracedNamespace>(node)
        {
            let mut namespace = ByteString::from(b"\\");
            namespace.extend(&value.bytes);
            context.set_namespace(namespace);
        }

        if let Some(UnbracedNamespace {
            name: SimpleIdentifier { value, .. },
            ..
        }) = downcast::<UnbracedNamespace>(node)
        {
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
                rule.run(
                    node,
                    &self.definitions,
                    &mut self.message_collector,
                    context,
                );
            }
        }

        Ok(())
    }
}
