use crate::{shared::modifier::Modifier, analyser::context::Context};
use pxp_parser::lexer::byte_string::ByteString;
use serde::{Deserialize, Serialize};

use super::{
    constants::ConstantDefinition, functions::MethodDefinition, property::PropertyDefinition, collection::DefinitionCollection,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ClassDefinition {
    pub name: ByteString,
    pub modifiers: Vec<Modifier>,
    pub extends: Option<ByteString>,
    pub implements: Vec<ByteString>,
    pub uses: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub properties: Vec<PropertyDefinition>,
    pub methods: Vec<MethodDefinition>,
}

impl ClassDefinition {
    pub fn is_abstract(&self) -> bool {
        self.modifiers.iter().any(|m| m == &Modifier::Abstract)
    }

    pub fn get_method(&self, name: &ByteString) -> Option<&MethodDefinition> {
        self.methods.iter().find(|m| m.name == *name)
    }

    pub fn get_inherited_method<'a>(&'a self, name: &ByteString, definitions: &'a DefinitionCollection, context: &Context) -> Option<(&'a ByteString, &'a MethodDefinition)> {
        // If we don't extend a class, then we can return early.
        if self.extends.is_none() {
            return None;
        }

        // Get the class we extend.
        let extends_class = self.extends.as_ref().unwrap();
        let extends = definitions.get_class(&extends_class, &context).unwrap();

        // Check if the class we extend has the method.
        let optional_method = extends.get_method(name);

        // If we found the method, return it.
        if optional_method.is_some() {
            return Some((extends_class, optional_method.unwrap()));
        }

        // Otherwise, we need to check if the parent class inherits the method.
        extends.get_inherited_method(name, definitions, context)
    }
}
