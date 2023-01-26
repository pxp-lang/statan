use pxp_parser::{
    downcast::downcast,
    lexer::byte_string::ByteString,
    node::Node,
    parser::ast::{
        classes::{ClassMember, ClassStatement},
        data_type::Type as ParsedType,
        enums::{BackedEnumMember, BackedEnumStatement, UnitEnumMember, UnitEnumStatement},
        functions::{FunctionStatement, ReturnType},
        identifiers::SimpleIdentifier,
        interfaces::{InterfaceMember, InterfaceStatement},
        modifiers::MethodModifier,
        namespaces::{BracedNamespace, UnbracedNamespace},
        traits::{TraitMember, TraitStatement},
        GroupUseStatement, Statement, Use, UseStatement,
    },
    traverser::Visitor,
};

use crate::shared::{modifier::Modifier, types::Type, visibility::Visibility};

use super::{
    classes::ClassDefinition,
    collection::DefinitionCollection,
    constants::ConstantDefinition,
    enums::EnumDefinition,
    functions::{FunctionDefinition, MethodDefinition},
    interfaces::InterfaceDefinition,
    parameter::Parameter,
    property::PropertyDefinition,
    traits::TraitDefinition,
};

#[derive(Debug)]
pub struct DefinitionCollector {
    current_namespace: ByteString,
    imported_names: Vec<ByteString>,
    collection: DefinitionCollection,
}

impl DefinitionCollector {
    pub fn new() -> Self {
        Self {
            current_namespace: ByteString::default(),
            imported_names: Vec::new(),
            collection: DefinitionCollection::default(),
        }
    }

    fn resolve_name(&self, name: &ByteString) -> ByteString {
        // If the name is already fully qualified, return as is.
        if name.bytes.starts_with(b"\\") {
            return name.clone();
        }

        let parts = name.split(|b| *b == b'\\').collect::<Vec<&[u8]>>();
        let first_part = parts.first().unwrap();

        // Check each imported name to see if it ends with the first part of the
        // given identifier. If it does, we can assume you're referencing either
        // an imported namespace or class that has been imported.
        for imported_name in self.imported_names.iter() {
            if imported_name.ends_with(first_part) {
                let mut qualified_name = imported_name.clone();
                qualified_name.extend(&name.bytes[first_part.len()..]);

                return qualified_name;
            }
        }

        // If we've reached this point, we have a simple name that
        // is not fully qualified and we have not imported it.
        // We can simply prepend the current namespace to it.
        let mut qualified_name = self.current_namespace.clone();
        qualified_name.extend(b"\\");
        qualified_name.extend(&name.bytes);

        qualified_name
    }

    fn qualify_name(&self, name: &ByteString) -> ByteString {
        let mut qualified_name = self.current_namespace.clone();
        qualified_name.extend(b"\\");
        qualified_name.extend(&name.bytes);

        qualified_name
    }

    fn map_type(&self, data_type: Option<&ParsedType>) -> Option<Type> {
        data_type.map(|t| match t {
            ParsedType::Named(_, name) => Type::Named(self.resolve_name(name)),
            ParsedType::Float(_) => Type::Float,
            ParsedType::Boolean(_) => Type::Bool,
            ParsedType::Integer(_) => Type::Int,
            ParsedType::String(_) => Type::String,
            ParsedType::Array(_) => Type::Array,
            ParsedType::Mixed(_) => Type::Mixed,
            _ => todo!(),
        })
    }

    pub fn collect(&self) -> DefinitionCollection {
        self.collection.clone()
    }

    pub fn scan(&mut self, ast: &mut [Statement]) {
        self.current_namespace = ByteString::default();
        self.imported_names = Vec::new();

        for statement in ast.iter_mut() {
            self.visit_node(statement).unwrap();
        }
    }
}

impl Visitor<()> for DefinitionCollector {
    fn visit(&mut self, node: &mut dyn Node) -> Result<(), ()> {
        if let Some(BracedNamespace {
            name: Some(SimpleIdentifier { value, .. }),
            ..
        }) = downcast::<BracedNamespace>(node)
        {
            let mut namespace = ByteString::from(b"\\");
            namespace.extend(&value.bytes);
            self.current_namespace = namespace;
        }

        if let Some(UnbracedNamespace {
            name: SimpleIdentifier { value, .. },
            ..
        }) = downcast::<UnbracedNamespace>(node)
        {
            let mut namespace = ByteString::from(b"\\");
            namespace.extend(&value.bytes);
            self.current_namespace = namespace;
        }

        if let Some(GroupUseStatement { prefix, uses, .. }) = downcast::<GroupUseStatement>(node) {
            for Use { name, .. } in uses {
                let mut prefixed_name = prefix.value.clone();
                prefixed_name.extend(b"\\");
                prefixed_name.extend(&name.value.bytes);

                self.imported_names.push(prefixed_name);
            }
        }

        if let Some(UseStatement { uses, .. }) = downcast::<UseStatement>(node) {
            for Use { name, .. } in uses {
                let mut qualified_name = ByteString::from(b"\\");
                qualified_name.extend(&name.value.bytes);
                self.imported_names.push(qualified_name);
            }
        }

        if let Some(FunctionStatement {
            name,
            parameters,
            return_type,
            ..
        }) = downcast::<FunctionStatement>(node)
        {
            let name = self.qualify_name(&name.value);
            let parameters = parameters
                .parameters
                .inner
                .iter()
                .map(|p| Parameter {
                    name: p.name.name.clone(),
                    type_: self.map_type(p.data_type.as_ref()),
                })
                .collect::<Vec<Parameter>>();
            let return_type = if let Some(ReturnType { data_type, .. }) = return_type {
                self.map_type(Some(data_type))
            } else {
                None
            };

            self.collection.add_function(FunctionDefinition {
                name,
                parameters,
                return_type,
            })
        }

        if let Some(ClassStatement {
            modifiers,
            name,
            extends,
            implements,
            body,
            ..
        }) = downcast::<ClassStatement>(node)
        {
            let modifiers = modifiers
                .modifiers
                .iter()
                .map(|m| m.clone().into())
                .collect::<Vec<Modifier>>();
            let name = self.qualify_name(&name.value);

            let extends = extends
                .as_ref()
                .map(|extends| self.resolve_name(&extends.parent.value));

            let implements = if let Some(implements) = implements {
                implements
                    .interfaces
                    .inner
                    .iter()
                    .map(|i| self.resolve_name(&i.value))
                    .collect::<Vec<ByteString>>()
            } else {
                Vec::new()
            };

            let uses = body
                .members
                .iter()
                .filter_map(|m| match m {
                    ClassMember::TraitUsage(usage) => Some(usage),
                    _ => None,
                })
                .flat_map(|m| {
                    m.traits
                        .iter()
                        .map(|i| self.resolve_name(&i.value))
                        .collect::<Vec<ByteString>>()
                })
                .collect::<Vec<ByteString>>();

            let constants = body
                .members
                .iter()
                .filter_map(|m| match m {
                    ClassMember::Constant(constant) => Some(constant),
                    _ => None,
                })
                .flat_map(|m| {
                    m.entries
                        .iter()
                        .map(|e| ConstantDefinition {
                            name: e.name.value.clone(),
                            visibility: m.modifiers.visibility().into(),
                            final_: m.modifiers.has_final(),
                        })
                        .collect::<Vec<ConstantDefinition>>()
                })
                .collect::<Vec<ConstantDefinition>>();

            let mut properties = body
                .members
                .iter()
                .filter_map(|m| match m {
                    ClassMember::Property(property) => Some(property),
                    _ => None,
                })
                .flat_map(|p| {
                    p.entries
                        .iter()
                        .map(|e| PropertyDefinition {
                            name: e.variable().name.clone(),
                            visibility: p.modifiers.visibility().into(),
                            modifier: if p.modifiers.has_readonly() {
                                Some(Modifier::Readonly)
                            } else if p.modifiers.has_static() {
                                Some(Modifier::Static)
                            } else {
                                None
                            },
                            type_: self.map_type(p.r#type.as_ref()),
                        })
                        .collect::<Vec<PropertyDefinition>>()
                })
                .collect::<Vec<PropertyDefinition>>();

            properties.extend(
                body.members
                    .iter()
                    .filter_map(|m| match m {
                        ClassMember::VariableProperty(property) => Some(property),
                        _ => None,
                    })
                    .flat_map(|p| {
                        p.entries
                            .iter()
                            .map(|e| PropertyDefinition {
                                name: e.variable().name.clone(),
                                visibility: Visibility::Public,
                                modifier: None,
                                type_: self.map_type(p.r#type.as_ref()),
                            })
                            .collect::<Vec<PropertyDefinition>>()
                    })
                    .collect::<Vec<PropertyDefinition>>(),
            );

            // TODO: Also add constructors to the method list.
            //       Ensure that any promoted properties from the constructor
            //       are also added to properties above. It might be easier to
            //       do this in a procedural loop.
            let mut methods = body
                .members
                .iter()
                .filter_map(|m| match m {
                    ClassMember::ConcreteMethod(method) => Some(method),
                    _ => None,
                })
                .map(|m| MethodDefinition {
                    name: m.name.value.clone(),
                    visibility: m.modifiers.visibility().into(),
                    modifiers: m
                        .modifiers
                        .modifiers
                        .iter()
                        .filter(|m| {
                            !matches!(
                                m,
                                MethodModifier::Public(_)
                                    | MethodModifier::Protected(_)
                                    | MethodModifier::Private(_)
                            )
                        })
                        .map(|m| m.clone().into())
                        .collect::<Vec<Modifier>>(),
                    parameters: m
                        .parameters
                        .parameters
                        .inner
                        .iter()
                        .map(|p| Parameter {
                            name: p.name.name.clone(),
                            type_: self.map_type(p.data_type.as_ref()),
                        })
                        .collect::<Vec<Parameter>>(),
                    return_type: if let Some(return_type) = &m.return_type {
                        self.map_type(Some(&return_type.data_type))
                    } else {
                        None
                    },
                })
                .collect::<Vec<MethodDefinition>>();

            methods.extend(
                body.members
                    .iter()
                    .filter_map(|m| match m {
                        ClassMember::AbstractMethod(method) => Some(method),
                        _ => None,
                    })
                    .map(|m| MethodDefinition {
                        name: m.name.value.clone(),
                        visibility: m.modifiers.visibility().into(),
                        modifiers: m
                            .modifiers
                            .modifiers
                            .iter()
                            .filter(|m| {
                                !matches!(
                                    m,
                                    MethodModifier::Public(_)
                                        | MethodModifier::Protected(_)
                                        | MethodModifier::Private(_)
                                )
                            })
                            .map(|m| m.clone().into())
                            .collect::<Vec<Modifier>>(),
                        parameters: m
                            .parameters
                            .parameters
                            .inner
                            .iter()
                            .map(|p| Parameter {
                                name: p.name.name.clone(),
                                type_: self.map_type(p.data_type.as_ref()),
                            })
                            .collect::<Vec<Parameter>>(),
                        return_type: if let Some(return_type) = &m.return_type {
                            self.map_type(Some(&return_type.data_type))
                        } else {
                            None
                        },
                    })
                    .collect::<Vec<MethodDefinition>>(),
            );

            self.collection.add_class(ClassDefinition {
                name,
                modifiers,
                extends,
                implements,
                uses,
                constants,
                properties,
                methods,
            });
        }

        if let Some(InterfaceStatement {
            name,
            extends,
            body,
            ..
        }) = downcast::<InterfaceStatement>(node)
        {
            let name = self.qualify_name(&name.value);
            let extends = if let Some(extends) = extends {
                extends
                    .parents
                    .inner
                    .iter()
                    .map(|i| self.resolve_name(&i.value))
                    .collect::<Vec<ByteString>>()
            } else {
                Vec::new()
            };
            let constants = body
                .members
                .iter()
                .filter_map(|m| match m {
                    InterfaceMember::Constant(constant) => Some(constant),
                    _ => None,
                })
                .flat_map(|m| {
                    m.entries
                        .iter()
                        .map(|e| ConstantDefinition {
                            name: e.name.value.clone(),
                            visibility: m.modifiers.visibility().into(),
                            final_: m.modifiers.has_final(),
                        })
                        .collect::<Vec<ConstantDefinition>>()
                })
                .collect::<Vec<ConstantDefinition>>();
            let methods = body
                .members
                .iter()
                .filter_map(|m| match m {
                    InterfaceMember::Method(method) => Some(method),
                    _ => None,
                })
                .map(|m| MethodDefinition {
                    name: m.name.value.clone(),
                    visibility: m.modifiers.visibility().into(),
                    modifiers: m
                        .modifiers
                        .modifiers
                        .iter()
                        .filter(|m| {
                            !matches!(
                                m,
                                MethodModifier::Public(_)
                                    | MethodModifier::Protected(_)
                                    | MethodModifier::Private(_)
                            )
                        })
                        .map(|m| m.clone().into())
                        .collect::<Vec<Modifier>>(),
                    parameters: m
                        .parameters
                        .parameters
                        .inner
                        .iter()
                        .map(|p| Parameter {
                            name: p.name.name.clone(),
                            type_: self.map_type(p.data_type.as_ref()),
                        })
                        .collect::<Vec<Parameter>>(),
                    return_type: if let Some(return_type) = &m.return_type {
                        self.map_type(Some(&return_type.data_type))
                    } else {
                        None
                    },
                })
                .collect::<Vec<MethodDefinition>>();

            self.collection.add_interface(InterfaceDefinition {
                name,
                extends,
                constants,
                methods,
            });
        }

        if let Some(TraitStatement { name, body, .. }) = downcast::<TraitStatement>(node) {
            let name = self.qualify_name(&name.value);

            let uses = body
                .members
                .iter()
                .filter_map(|m| match m {
                    TraitMember::TraitUsage(usage) => Some(usage),
                    _ => None,
                })
                .flat_map(|m| {
                    m.traits
                        .iter()
                        .map(|i| self.resolve_name(&i.value))
                        .collect::<Vec<ByteString>>()
                })
                .collect::<Vec<ByteString>>();

            let constants = body
                .members
                .iter()
                .filter_map(|m| match m {
                    TraitMember::Constant(constant) => Some(constant),
                    _ => None,
                })
                .flat_map(|m| {
                    m.entries
                        .iter()
                        .map(|e| ConstantDefinition {
                            name: e.name.value.clone(),
                            visibility: m.modifiers.visibility().into(),
                            final_: m.modifiers.has_final(),
                        })
                        .collect::<Vec<ConstantDefinition>>()
                })
                .collect::<Vec<ConstantDefinition>>();

            let mut properties = body
                .members
                .iter()
                .filter_map(|m| match m {
                    TraitMember::Property(property) => Some(property),
                    _ => None,
                })
                .flat_map(|p| {
                    p.entries
                        .iter()
                        .map(|e| PropertyDefinition {
                            name: e.variable().name.clone(),
                            visibility: p.modifiers.visibility().into(),
                            modifier: if p.modifiers.has_readonly() {
                                Some(Modifier::Readonly)
                            } else if p.modifiers.has_static() {
                                Some(Modifier::Static)
                            } else {
                                None
                            },
                            type_: self.map_type(p.r#type.as_ref()),
                        })
                        .collect::<Vec<PropertyDefinition>>()
                })
                .collect::<Vec<PropertyDefinition>>();

            properties.extend(
                body.members
                    .iter()
                    .filter_map(|m| match m {
                        TraitMember::VariableProperty(property) => Some(property),
                        _ => None,
                    })
                    .flat_map(|p| {
                        p.entries
                            .iter()
                            .map(|e| PropertyDefinition {
                                name: e.variable().name.clone(),
                                visibility: Visibility::Public,
                                modifier: None,
                                type_: self.map_type(p.r#type.as_ref()),
                            })
                            .collect::<Vec<PropertyDefinition>>()
                    })
                    .collect::<Vec<PropertyDefinition>>(),
            );

            // TODO: Also add constructors to the method list.
            //       Ensure that any promoted properties from the constructor
            //       are also added to properties above. It might be easier to
            //       do this in a procedural loop.
            let mut methods = body
                .members
                .iter()
                .filter_map(|m| match m {
                    TraitMember::ConcreteMethod(method) => Some(method),
                    _ => None,
                })
                .map(|m| MethodDefinition {
                    name: m.name.value.clone(),
                    visibility: m.modifiers.visibility().into(),
                    modifiers: m
                        .modifiers
                        .modifiers
                        .iter()
                        .filter(|m| {
                            !matches!(
                                m,
                                MethodModifier::Public(_)
                                    | MethodModifier::Protected(_)
                                    | MethodModifier::Private(_)
                            )
                        })
                        .map(|m| m.clone().into())
                        .collect::<Vec<Modifier>>(),
                    parameters: m
                        .parameters
                        .parameters
                        .inner
                        .iter()
                        .map(|p| Parameter {
                            name: p.name.name.clone(),
                            type_: self.map_type(p.data_type.as_ref()),
                        })
                        .collect::<Vec<Parameter>>(),
                    return_type: if let Some(return_type) = &m.return_type {
                        self.map_type(Some(&return_type.data_type))
                    } else {
                        None
                    },
                })
                .collect::<Vec<MethodDefinition>>();

            methods.extend(
                body.members
                    .iter()
                    .filter_map(|m| match m {
                        TraitMember::AbstractMethod(method) => Some(method),
                        _ => None,
                    })
                    .map(|m| MethodDefinition {
                        name: m.name.value.clone(),
                        visibility: m.modifiers.visibility().into(),
                        modifiers: m
                            .modifiers
                            .modifiers
                            .iter()
                            .filter(|m| {
                                !matches!(
                                    m,
                                    MethodModifier::Public(_)
                                        | MethodModifier::Protected(_)
                                        | MethodModifier::Private(_)
                                )
                            })
                            .map(|m| m.clone().into())
                            .collect::<Vec<Modifier>>(),
                        parameters: m
                            .parameters
                            .parameters
                            .inner
                            .iter()
                            .map(|p| Parameter {
                                name: p.name.name.clone(),
                                type_: self.map_type(p.data_type.as_ref()),
                            })
                            .collect::<Vec<Parameter>>(),
                        return_type: if let Some(return_type) = &m.return_type {
                            self.map_type(Some(&return_type.data_type))
                        } else {
                            None
                        },
                    })
                    .collect::<Vec<MethodDefinition>>(),
            );

            self.collection.add_trait(TraitDefinition {
                name,
                uses,
                constants,
                properties,
                methods,
            });
        }

        if let Some(UnitEnumStatement {
            name,
            implements,
            body,
            ..
        }) = downcast::<UnitEnumStatement>(node)
        {
            let name = self.qualify_name(&name.value);
            let implements = implements
                .iter()
                .map(|i| self.resolve_name(&i.value))
                .collect::<Vec<ByteString>>();

            let constants = body
                .members
                .iter()
                .filter_map(|m| match m {
                    UnitEnumMember::Constant(constant) => Some(constant),
                    _ => None,
                })
                .flat_map(|m| {
                    m.entries
                        .iter()
                        .map(|e| ConstantDefinition {
                            name: e.name.value.clone(),
                            visibility: m.modifiers.visibility().into(),
                            final_: m.modifiers.has_final(),
                        })
                        .collect::<Vec<ConstantDefinition>>()
                })
                .collect::<Vec<ConstantDefinition>>();

            let methods = body
                .members
                .iter()
                .filter_map(|m| match m {
                    UnitEnumMember::Method(method) => Some(method),
                    _ => None,
                })
                .map(|m| MethodDefinition {
                    name: m.name.value.clone(),
                    visibility: m.modifiers.visibility().into(),
                    modifiers: m
                        .modifiers
                        .modifiers
                        .iter()
                        .filter(|m| {
                            !matches!(
                                m,
                                MethodModifier::Public(_)
                                    | MethodModifier::Protected(_)
                                    | MethodModifier::Private(_)
                            )
                        })
                        .map(|m| m.clone().into())
                        .collect::<Vec<Modifier>>(),
                    parameters: m
                        .parameters
                        .parameters
                        .inner
                        .iter()
                        .map(|p| Parameter {
                            name: p.name.name.clone(),
                            type_: self.map_type(p.data_type.as_ref()),
                        })
                        .collect::<Vec<Parameter>>(),
                    return_type: if let Some(return_type) = &m.return_type {
                        self.map_type(Some(&return_type.data_type))
                    } else {
                        None
                    },
                })
                .collect::<Vec<MethodDefinition>>();

            let members = body
                .members
                .iter()
                .filter_map(|m| match m {
                    UnitEnumMember::Case(member) => Some(member),
                    _ => None,
                })
                .map(|c| c.name.value.clone())
                .collect::<Vec<ByteString>>();

            self.collection.add_enum(EnumDefinition {
                name,
                implements,
                constants,
                methods,
                members,
                backed_type: None,
            });
        }

        if let Some(BackedEnumStatement {
            name,
            implements,
            body,
            backed_type,
            ..
        }) = downcast::<BackedEnumStatement>(node)
        {
            let name = self.qualify_name(&name.value);
            let implements = implements
                .iter()
                .map(|i| self.resolve_name(&i.value))
                .collect::<Vec<ByteString>>();

            let constants = body
                .members
                .iter()
                .filter_map(|m| match m {
                    BackedEnumMember::Constant(constant) => Some(constant),
                    _ => None,
                })
                .flat_map(|m| {
                    m.entries
                        .iter()
                        .map(|e| ConstantDefinition {
                            name: e.name.value.clone(),
                            visibility: m.modifiers.visibility().into(),
                            final_: m.modifiers.has_final(),
                        })
                        .collect::<Vec<ConstantDefinition>>()
                })
                .collect::<Vec<ConstantDefinition>>();

            let methods = body
                .members
                .iter()
                .filter_map(|m| match m {
                    BackedEnumMember::Method(method) => Some(method),
                    _ => None,
                })
                .map(|m| MethodDefinition {
                    name: m.name.value.clone(),
                    visibility: m.modifiers.visibility().into(),
                    modifiers: m
                        .modifiers
                        .modifiers
                        .iter()
                        .filter(|m| {
                            !matches!(
                                m,
                                MethodModifier::Public(_)
                                    | MethodModifier::Protected(_)
                                    | MethodModifier::Private(_)
                            )
                        })
                        .map(|m| m.clone().into())
                        .collect::<Vec<Modifier>>(),
                    parameters: m
                        .parameters
                        .parameters
                        .inner
                        .iter()
                        .map(|p| Parameter {
                            name: p.name.name.clone(),
                            type_: self.map_type(p.data_type.as_ref()),
                        })
                        .collect::<Vec<Parameter>>(),
                    return_type: if let Some(return_type) = &m.return_type {
                        self.map_type(Some(&return_type.data_type))
                    } else {
                        None
                    },
                })
                .collect::<Vec<MethodDefinition>>();

            let members = body
                .members
                .iter()
                .filter_map(|m| match m {
                    BackedEnumMember::Case(member) => Some(member),
                    _ => None,
                })
                .map(|c| c.name.value.clone())
                .collect::<Vec<ByteString>>();

            self.collection.add_enum(EnumDefinition {
                name,
                implements,
                constants,
                methods,
                members,
                backed_type: Some(backed_type.clone().into()),
            });
        }

        Ok(())
    }
}

impl Default for DefinitionCollector {
    fn default() -> Self {
        Self::new()
    }
}
