use pxp_parser::{lexer::byte_string::ByteString, parser::ast::{Statement, namespaces::{BracedNamespace, UnbracedNamespace}, identifiers::SimpleIdentifier, GroupUseStatement, UseStatement, Use}, traverser::Visitor, node::Node, downcast::{self, downcast}};

#[derive(Debug)]
pub struct DefinitionCollector {
    current_namespace: ByteString,
    imported_names: Vec<ByteString>,
}

impl DefinitionCollector {
    pub fn new() -> Self {
        Self {
            current_namespace: ByteString::default(),
            imported_names: Vec::new(),
        }
    }

    pub fn scan(&mut self, ast: &mut Vec<Statement>) {

    }
}

impl Visitor<()> for DefinitionCollector {
    fn visit(&mut self, node: &mut dyn Node) -> Result<(), ()> {
        if let Some(BracedNamespace { name: Some(SimpleIdentifier { value, .. }), .. }) = downcast::<BracedNamespace>(node) {
            self.current_namespace = value.clone();
        }
        
        if let Some(UnbracedNamespace { name: SimpleIdentifier { value, .. }, .. }) = downcast::<UnbracedNamespace>(node) {
            self.current_namespace = value.clone();
        }

        if let Some(GroupUseStatement { prefix, uses, .. }) = downcast::<GroupUseStatement>(node) {
            for Use { name, .. } in uses {
                let mut prefixed_name = prefix.value.clone();
                prefixed_name.extend(b"\\");
                prefixed_name.extend(&name.value.bytes);

                self.imported_names.push(prefixed_name);
            }
        }

        if let Some(UseStatement { uses, .. }) = downcast::<UseStatement>(node) {
            for Use { name, .. } in uses {
                self.imported_names.push(name.value.clone());
            }
        }

        Ok(())
    }
}