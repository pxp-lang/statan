use pxp_parser::lexer::byte_string::ByteString;
use serde::{Serialize, Deserialize};

use crate::analyser::context::Context;

use super::{
    constants::ConstantDefinition, functions::MethodDefinition, property::PropertyDefinition, collection::DefinitionCollection,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitDefinition {
    pub name: ByteString,
    pub uses: Vec<ByteString>,
    pub constants: Vec<ConstantDefinition>,
    pub properties: Vec<PropertyDefinition>,
    pub methods: Vec<MethodDefinition>,
}

impl TraitDefinition {
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
}