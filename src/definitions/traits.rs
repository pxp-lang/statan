use crate::shared::modifier::Modifier;
use pxp_parser::lexer::byte_string::ByteString;

use super::{
    constants::ConstantDefinition, functions::MethodDefinition, property::PropertyDefinition,
};

#[derive(Debug, Clone)]
pub struct TraitDefinition {
    pub name: ByteString,
    pub uses: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub properties: Vec<PropertyDefinition>,
    pub methods: Vec<MethodDefinition>,
}
