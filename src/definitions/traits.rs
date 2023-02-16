use pxp_parser::lexer::byte_string::ByteString;
use serde::{Serialize, Deserialize};

use super::{
    constants::ConstantDefinition, functions::MethodDefinition, property::PropertyDefinition,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitDefinition {
    pub name: ByteString,
    pub uses: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub properties: Vec<PropertyDefinition>,
    pub methods: Vec<MethodDefinition>,
}
