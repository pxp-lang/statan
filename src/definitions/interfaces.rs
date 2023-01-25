use pxp_parser::lexer::byte_string::ByteString;

use super::{functions::MethodDefinition, constants::ConstantDefinition};

#[derive(Debug, Clone)]
pub struct InterfaceDefinition {
    pub name: ByteString,
    pub extends: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub methods: Vec<MethodDefinition>,
}