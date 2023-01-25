use pxp_parser::lexer::byte_string::ByteString;

use super::{constants::ConstantDefinition, functions::MethodDefinition};

#[derive(Debug, Clone)]
pub struct InterfaceDefinition {
    pub name: ByteString,
    pub extends: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub methods: Vec<MethodDefinition>,
}
