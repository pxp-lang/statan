use pxp_parser::lexer::byte_string::ByteString;
use serde::{Serialize, Deserialize};

use crate::shared::{modifier::Modifier, types::Type, visibility::Visibility};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    pub name: ByteString,
    pub visibility: Visibility,
    pub modifier: Option<Modifier>,
    pub type_: Option<Type>,
}
