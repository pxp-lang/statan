use pxp_parser::{lexer::byte_string::ByteString, parser::ast::enums::BackedEnumType};

use super::{functions::MethodDefinition, constants::ConstantDefinition};

#[derive(Debug, Clone)]
pub struct EnumDefinition {
    pub name: ByteString,
    pub implements: Vec<ByteString>,
    pub backed_type: Option<EnumBackedType>,
    pub members: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub methods: Vec<MethodDefinition>,
}

#[derive(Debug, Clone)]
pub enum EnumBackedType {
    Int,
    String,
}

impl From<BackedEnumType> for EnumBackedType {
    fn from(value: BackedEnumType) -> Self {
        match value {
            BackedEnumType::String(_, _) => Self::String,
            BackedEnumType::Int(_, _) => Self::Int,
        }
    }
}