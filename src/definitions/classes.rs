use pxp_parser::lexer::byte_string::ByteString;
use crate::shared::modifier::Modifier;

use super::{property::PropertyDefinition, functions::MethodDefinition, constants::ConstantDefinition};

#[derive(Debug, Clone)]
pub struct ClassDefinition {
    pub name: ByteString,
    pub modifiers: Vec<Modifier>,
    pub extends: Option<ByteString>,
    pub implements: Vec<ByteString>,
    pub uses: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub properties: Vec<PropertyDefinition>,
    pub methods: Vec<MethodDefinition>,
}