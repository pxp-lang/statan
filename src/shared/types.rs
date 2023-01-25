use pxp_parser::lexer::byte_string::ByteString;

#[derive(Debug, Clone)]
pub enum Type {
    String,
    Int,
    Float,
    Array,
    Mixed,
    Bool,   
    Named(ByteString),
}