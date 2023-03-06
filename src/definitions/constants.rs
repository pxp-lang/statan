use pxp_parser::lexer::byte_string::ByteString;
use serde::{Deserialize, Serialize};

use crate::shared::visibility::Visibility;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstantDefinition {
    pub name: ByteString,
    pub visibility: Visibility,
    pub final_: bool,
}
