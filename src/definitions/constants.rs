use pxp_parser::lexer::byte_string::ByteString;

use crate::shared::{modifier::Modifier, visibility::Visibility};

#[derive(Debug, Clone)]
pub struct ConstantDefinition {
    pub name: ByteString,
    pub visibility: Visibility,
    pub final_: bool,
}