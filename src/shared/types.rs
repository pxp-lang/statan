use std::fmt::{Debug, Display};

use pxp_parser::lexer::byte_string::ByteString;
use serde::{Serialize, Deserialize};

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
            Type::Nullable(ty) => write!(f, "?{}", ty),
            Type::Named(ty) => write!(f, "{}", ty),
            Type::Union(tys) => write!(f, "{}", tys.iter().map(|ty| ty.to_string()).collect::<Vec<String>>().join("|")),
            Type::Intersection(tys) => write!(f, "{}", tys.iter().map(|ty| ty.to_string()).collect::<Vec<String>>().join("&")),
            Type::Never => write!(f, "never"),
        }
    }
}
