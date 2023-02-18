use pxp_parser::lexer::byte_string::ByteString;
use serde::{Serialize, Deserialize};

use crate::analyser::context::Context;

use super::{
    classes::ClassDefinition, enums::EnumDefinition, functions::FunctionDefinition,
    interfaces::InterfaceDefinition, traits::TraitDefinition,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DefinitionCollection {
    functions: Vec<FunctionDefinition>,
    classes: Vec<ClassDefinition>,
    interfaces: Vec<InterfaceDefinition>,
    traits: Vec<TraitDefinition>,
    enums: Vec<EnumDefinition>,
}

impl DefinitionCollection {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            classes: Vec::new(),
            interfaces: Vec::new(),
            traits: Vec::new(),
            enums: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: FunctionDefinition) {
        self.functions.push(function);
    }

    pub fn add_class(&mut self, class: ClassDefinition) {
        self.classes.push(class);
    }

    pub fn add_interface(&mut self, interface: InterfaceDefinition) {
        self.interfaces.push(interface);
    }

    pub fn add_trait(&mut self, trait_: TraitDefinition) {
        self.traits.push(trait_);
    }

    pub fn add_enum(&mut self, enum_: EnumDefinition) {
        self.enums.push(enum_);
    }

    pub fn get_function(&self, name: &ByteString, context: &Context) -> Option<&FunctionDefinition> {
        let resolved_name = context.resolve_name(name);
        
        self.functions.iter()
            .find(|function| function.name == resolved_name)
            .or_else(|| {
                let mut global_name = ByteString::default();
                global_name.extend(b"\\");
                global_name.extend(&name.bytes);

                self.functions.iter()
                    .find(|function| function.name == global_name)
            })
    }

    pub fn get_class(&self, name: &ByteString, context: &Context) -> Option<&ClassDefinition> {
        let resolved_name = context.resolve_name(name);
        
        self.classes.iter()
            .find(|class| class.name == resolved_name)
            .or_else(|| {
                let mut global_name = ByteString::default();
                global_name.extend(b"\\");
                global_name.extend(&name.bytes);

                self.classes.iter()
                    .find(|class| class.name == global_name)
            })
    }

    pub fn get_trait(&self, name: &ByteString, context: &Context) -> Option<&TraitDefinition> {
        let resolved_name = context.resolve_name(name);
        
        self.traits.iter()
            .find(|trait_| trait_.name == resolved_name)
            .or_else(|| {
                let mut global_name = ByteString::default();
                global_name.extend(b"\\");
                global_name.extend(&name.bytes);

                self.traits.iter()
                    .find(|trait_| trait_.name == global_name)
            })
    }
}
