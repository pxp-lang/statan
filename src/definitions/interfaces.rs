use pxp_parser::lexer::byte_string::ByteString;
use serde::{Deserialize, Serialize};

use super::{constants::ConstantDefinition, functions::MethodDefinition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceDefinition {
    pub name: ByteString,
    pub extends: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub methods: Vec<MethodDefinition>,
}
