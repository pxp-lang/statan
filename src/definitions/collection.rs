use pxp_parser::lexer::byte_string::ByteString;

use super::{
    classes::ClassDefinition, enums::EnumDefinition, functions::FunctionDefinition,
    interfaces::InterfaceDefinition, traits::TraitDefinition,
};

#[derive(Debug, Default, Clone)]
pub struct DefinitionCollection {
    functions: Vec<FunctionDefinition>,
    classes: Vec<ClassDefinition>,
    interfaces: Vec<InterfaceDefinition>,
    traits: Vec<TraitDefinition>,
    enums: Vec<EnumDefinition>,
}

impl DefinitionCollection {
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

    pub fn get_function(&self, name: &ByteString) -> Option<&FunctionDefinition> {
        self.functions.iter().find(|function| &function.name == name)
    }
}
