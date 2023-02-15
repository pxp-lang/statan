use pxp_parser::lexer::byte_string::ByteString;

#[derive(Debug, Clone)]
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
