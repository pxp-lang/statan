use std::fmt::{Debug, Display};

use pxp_parser::{lexer::byte_string::ByteString, parser::ast::data_type::Type as ParsedType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type {
    String,
    Int,
    Float,
    Array,
    Mixed,
    Bool,
    Object,
    Void,
    False,
    True,
    Null,
    Callable,
    Static,
    Self_,
    Parent,
    Iterable,
    Nullable(Box<Self>),
    Named(ByteString),
    Union(Vec<Self>),
    Intersection(Vec<Self>),
    Never,
    Error,
}

impl From<&ParsedType> for Type {
    fn from(value: &ParsedType) -> Self {
        value.clone().into()
    }
}

impl From<ParsedType> for Type {
    fn from(value: ParsedType) -> Self {
        match value {
            ParsedType::Named(_, t) => Self::Named(t),
            ParsedType::Nullable(_, t) => Self::Nullable(Box::new(t.as_ref().into())),
            ParsedType::Union(tys) => Self::Union(tys.into_iter().map(|t| t.into()).collect()),
            ParsedType::Intersection(tys) => {
                Self::Intersection(tys.into_iter().map(|t| t.into()).collect())
            }
            ParsedType::Void(_) => Self::Void,
            ParsedType::Null(_) => Self::Null,
            ParsedType::True(_) => Self::True,
            ParsedType::False(_) => Self::False,
            ParsedType::Never(_) => Self::Never,
            ParsedType::Float(_) => Self::Float,
            ParsedType::Boolean(_) => Self::Bool,
            ParsedType::Integer(_) => Self::Int,
            ParsedType::String(_) => Self::String,
            ParsedType::Array(_) => Self::Array,
            ParsedType::Object(_) => Self::Object,
            ParsedType::Mixed(_) => Self::Mixed,
            ParsedType::Callable(_) => Self::Callable,
            ParsedType::Iterable(_) => Self::Iterable,
            ParsedType::StaticReference(_) => Self::Static,
            ParsedType::SelfReference(_) => Self::Self_,
            ParsedType::ParentReference(_) => Self::Parent,
        }
    }
}

impl Type {
    pub fn compatible(&self, other: &Type) -> bool {
        if other == &Type::Mixed {
            return true;
        }

        match self {
            Type::String => other == &Type::String,
            Type::Int => other == &Type::Int,
            Type::Float => other == &Type::Float,
            Type::Array => other == &Type::Array,
            Type::Mixed => true,
            Type::Bool => other == &Type::Bool || other == &Type::True || other == &Type::False,
            Type::Object => other == &Type::Object || matches!(other, Type::Named(_)),
            Type::Void => other == &Type::Void || other == &Type::Null,
            Type::False => other == &Type::False,
            Type::True => other == &Type::True,
            Type::Null => other == &Type::Null || matches!(other, Type::Nullable(_)),
            Type::Callable => other == &Type::Callable,
            Type::Static => todo!(),
            Type::Self_ => todo!(),
            Type::Parent => todo!(),
            // FIXME: Add a \Traversable check here too.
            Type::Iterable => other == &Type::Iterable,
            Type::Nullable(ty) => other == &Type::Null || ty.compatible(other),
            // FIXME: Also need to check variance of the type as well.
            Type::Named(_) => self == other,
            Type::Union(ty) => ty.iter().any(|ty| ty.compatible(other)),
            Type::Intersection(ty) => ty.iter().all(|ty| ty.compatible(other)),
            Type::Never => false,
            Type::Error => unreachable!(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String => write!(f, "string"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Array => write!(f, "array"),
            Type::Mixed => write!(f, "mixed"),
            Type::Bool => write!(f, "bool"),
            Type::Object => write!(f, "object"),
            Type::Void => write!(f, "void"),
            Type::False => write!(f, "false"),
            Type::True => write!(f, "true"),
            Type::Null => write!(f, "null"),
            Type::Callable => write!(f, "callable"),
            Type::Static => write!(f, "static"),
            Type::Self_ => write!(f, "self"),
            Type::Parent => write!(f, "parent"),
            Type::Iterable => write!(f, "iterable"),
            Type::Nullable(ty) => write!(f, "?{ty}"),
            Type::Named(ty) => write!(f, "{ty}",),
            Type::Union(tys) => write!(
                f,
                "{}",
                tys.iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<String>>()
                    .join("|")
            ),
            Type::Intersection(tys) => write!(
                f,
                "{}",
                tys.iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<String>>()
                    .join("&")
            ),
            Type::Never => write!(f, "never"),
            Type::Error => write!(f, "<internal:error>"),
        }
    }
}
