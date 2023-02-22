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

    pub fn is_final(&self) -> bool {
        self.modifiers.iter().any(|m| m == &Modifier::Final)
    }

    pub fn get_method<'a>(&'a self, name: &ByteString, definitions: &'a DefinitionCollection, context: &Context) -> Option<&'a MethodDefinition> {
        self.methods.iter()
            .find(|m| m.name == *name)
            .or_else(|| {
                for trait_ in &self.uses {
                    let trait_ = definitions.get_trait(trait_, context);

                    if trait_.is_none() {
                        continue;
                    }

                    let trait_ = trait_.unwrap();
                    let method = trait_.get_method(name, definitions, context);

                    if method.is_some() {
                        return method;
                    }
                }

                None
            })
    }

    pub fn get_inherited_method<'a>(&'a self, name: &ByteString, definitions: &'a DefinitionCollection, context: &Context) -> Option<(&'a ByteString, &'a MethodDefinition)> {
        // If we don't extend a class, then we can return early.
        self.extends.as_ref()?;

        // Get the class we extend.
        let extends_class = self.extends.as_ref().unwrap();
        let extends = definitions.get_class(extends_class, context);

        extends?;

        let extends = extends.unwrap();

        // Check if the class we extend has the method.
        let optional_method = extends.get_method(name, definitions, context);

        // If we found the method, return it.
        if let Some(method) = optional_method {
            return Some((extends_class, method));
        }

        // Otherwise, we need to check if the parent class inherits the method.
        extends.get_inherited_method(name, definitions, context)
    }
}
