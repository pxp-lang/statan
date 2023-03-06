use pxp_parser::lexer::byte_string::ByteString;
use serde::{Deserialize, Serialize};

use crate::shared::{modifier::Modifier, types::Type, visibility::Visibility};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyDefinition {
    pub name: ByteString,
    pub visibility: Visibility,
    pub modifier: Option<Modifier>,
    pub type_: Option<Type>,
}
