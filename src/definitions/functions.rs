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

    pub fn min_arity(&self) -> usize {
        self.parameters.iter().take_while(|p| !p.optional && !p.spread).count()
    }

    pub fn max_arity(&self) -> usize {
        if self.parameters.iter().any(|p| p.spread) {
            usize::MAX
        } else {
            self.parameters.len()
        }
    }

    pub fn get_parameter_by_position(&self, position: usize) -> Option<&Parameter> {
        if position >= self.parameters.len() {
            self.parameters.last()
        } else {
            self.parameters.get(position)
        }
    }

    pub fn get_parameter_by_name(&self, name: &ByteString) -> Option<&Parameter> {
        let mut name = name.clone();

        if ! name.starts_with(&[b'$']) {
            name.bytes.insert(0, b'$');
        }

        self.parameters.iter().find(|p| p.name == name)
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