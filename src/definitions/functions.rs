use pxp_parser::lexer::byte_string::ByteString;

use crate::shared::{types::Type, visibility::Visibility, modifier::Modifier};

use super::parameter::Parameter;

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: ByteString,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct MethodDefinition {
    pub name: ByteString,
    pub visibility: Visibility,
    pub modifiers: Vec<Modifier>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}