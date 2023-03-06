use pxp_parser::parser::ast::modifiers::Visibility as ParsedVisibility;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl From<ParsedVisibility> for Visibility {
    fn from(value: ParsedVisibility) -> Self {
        match value {
            ParsedVisibility::Public => Self::Public,
            ParsedVisibility::Protected => Self::Protected,
            ParsedVisibility::Private => Self::Private,
        }
    }
}
