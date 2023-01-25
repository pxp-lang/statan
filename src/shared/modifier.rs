use pxp_parser::parser::ast::modifiers::{ClassModifier, MethodModifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
    Final,
    Static,
    Abstract,
    Readonly,
}

impl From<ClassModifier> for Modifier {
    fn from(value: ClassModifier) -> Self {
        match value {
            ClassModifier::Final(_) => Self::Final,
            ClassModifier::Abstract(_) => Self::Abstract,
            ClassModifier::Readonly(_) => Self::Readonly,
        }
    }
}

impl From<MethodModifier> for Modifier {
    fn from(value: MethodModifier) -> Self {
        match value {
            MethodModifier::Final(_) => Self::Final,
            MethodModifier::Static(_) => Self::Static,
            MethodModifier::Abstract(_) => Self::Abstract,
            _ => unreachable!(),
        }
    }
}