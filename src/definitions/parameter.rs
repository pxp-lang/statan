use pxp_parser::lexer::byte_string::ByteString;

use crate::shared::types::Type;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: ByteString,
    pub type_: Option<Type>,
}