use crate::shared::modifier::Modifier;
use pxp_parser::lexer::byte_string::ByteString;
use serde::{Deserialize, Serialize};

use super::{
    constants::ConstantDefinition, functions::MethodDefinition, property::PropertyDefinition,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl ClassDefinition {
    pub fn is_abstract(&self) -> bool {
        self.modifiers.iter().any(|m| m == &Modifier::Abstract)
    }
}
