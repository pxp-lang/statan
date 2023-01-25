use pxp_parser::parser::ast::modifiers::Visibility as ParsedVisibility;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl From<ParsedVisibility> for Visibility {
    fn from(value: ParsedVisibility) -> Self {
        match value {
            ParsedVisibility::Public => Self::Public,
            ParsedVisibility::Protected => Self::Protected,
            ParsedVisibility::Private => Self::Private,
        }
    }
}