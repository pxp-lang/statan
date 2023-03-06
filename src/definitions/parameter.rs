use pxp_parser::lexer::byte_string::ByteString;
use serde::{Deserialize, Serialize};

use crate::shared::types::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: ByteString,
    pub type_: Option<Type>,
    pub optional: bool,
    pub spread: bool,
}
