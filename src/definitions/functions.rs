use pxp_parser::lexer::byte_string::ByteString;
use serde::{Serialize, Deserialize};

use crate::shared::{modifier::Modifier, types::Type, visibility::Visibility};

use super::parameter::Parameter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: ByteString,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

impl FunctionDefinition {
    pub fn returns_void(&self) -> bool {
        matches!(self.return_type, Some(Type::Void))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MethodDefinition {
    pub name: ByteString,
    pub visibility: Visibility,
    pub modifiers: Vec<Modifier>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

impl MethodDefinition {
    pub fn is_static(&self) -> bool {
        self.modifiers.iter().any(|m| m == &Modifier::Static)
    }

    pub fn is_abstract(&self) -> bool {
        self.modifiers.iter().any(|m| m == &Modifier::Abstract)
    }

    pub fn is_public(&self) -> bool {
        self.visibility == Visibility::Public
    }

    pub fn is_protected(&self) -> bool {
        self.visibility == Visibility::Protected
    }

    pub fn is_private(&self) -> bool {
        self.visibility == Visibility::Private
    }
}