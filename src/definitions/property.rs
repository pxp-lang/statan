use pxp_parser::lexer::byte_string::ByteString;

use crate::shared::{visibility::Visibility, modifier::Modifier, types::Type};

#[derive(Debug, Clone)]
pub struct PropertyDefinition {
    pub name: ByteString,
    pub visibility: Visibility,
    pub modifier: Option<Modifier>,
    pub type_: Option<Type>,
}