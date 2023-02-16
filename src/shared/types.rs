use pxp_parser::lexer::byte_string::ByteString;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
