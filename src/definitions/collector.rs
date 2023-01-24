use pxp_parser::{lexer::byte_string::ByteString, parser::ast::Statement};

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