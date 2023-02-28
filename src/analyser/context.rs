use std::collections::HashMap;

use pxp_parser::{lexer::byte_string::ByteString, parser::ast::{Expression, literals::Literal, variables::{Variable, SimpleVariable}, FunctionCallExpression, identifiers::{Identifier, SimpleIdentifier}, NewExpression, operators::ArithmeticOperationExpression}};
use crate::{shared::types::Type, definitions::collection::DefinitionCollection};

#[derive(Debug, Clone)]
pub struct Context {
    namespace: ByteString,
    imports: Vec<ByteString>,
    variables: HashMap<ByteString, Type>,
    classish_context: Option<ByteString>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            namespace: ByteString::default(),
            imports: Vec::new(),
            variables: HashMap::new(),
            classish_context: None,
        }
    }

    pub fn set_variable(&mut self, name: ByteString, ty: Type) {
        self.variables.insert(name, ty);
    }

    pub fn has_variable(&self, name: &ByteString) -> bool {
        self.variables.contains_key(name)
    }

    pub fn clean(&self) -> Self {
        Self {
            namespace: self.namespace.clone(),
            imports: self.imports.clone(),
            variables: HashMap::new(),
            classish_context: self.classish_context.clone(),
        }
    }

    pub fn set_classish_context(&mut self, name: &ByteString) {
        self.classish_context = Some(name.clone());
    }

    pub fn is_in_class(&self) -> bool {
        self.classish_context.is_some()
    }

    pub fn classish_context(&self) -> &ByteString {
        self.classish_context.as_ref().unwrap()
    }

    pub fn get_type(&self, expression: &Expression, definitions: &DefinitionCollection) -> Type {
        match expression {
            Expression::Literal(Literal::Integer(_)) => Type::Int,
            Expression::Literal(Literal::Float(_)) => Type::Float,
            Expression::Literal(Literal::String(_)) => Type::String,
            Expression::Bool(_) => Type::Bool,
            Expression::Null => Type::Null,
            Expression::Variable(Variable::SimpleVariable(SimpleVariable { name, .. })) => {
                self.variables.get(name).cloned().unwrap_or(Type::Mixed)
            },
            Expression::FunctionCall(FunctionCallExpression { target, .. }) => match target.as_ref() {
                Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { value: function_name, .. })) => {
                    if let Some(function_definition) = definitions.get_function(function_name, self) {
                        if let Some(return_type) = function_definition.return_type.as_ref() {
                            return_type.clone()
                        } else {
                            Type::Mixed
                        }
                    } else {
                        // NOTE: If we reach this point, we can't find the function so we'll let the valid function rule
                        //       take care of the error.
                        Type::Mixed
                    }
                },
                _ => Type::Mixed,
            },
            Expression::New(NewExpression { target, .. }) => {
                match target.as_ref() {
                    Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { value, .. })) => {
                        let class = definitions.get_class(value, self).unwrap();
                        Type::Named(class.name.clone())
                    },
                    _ => Type::Object,
                }
            },
            Expression::ArithmeticOperation(operation) => match operation {
                ArithmeticOperationExpression::Addition { left, right, .. } => match (self.get_type(left.as_ref(), definitions), self.get_type(right.as_ref(), definitions)) {
                    (Type::Float, Type::Int | Type::Float) => Type::Float,
                    (Type::Int, Type::Float) => Type::Float,
                    (Type::Int, Type::Int) => Type::Int,
                    (Type::Array, Type::Array) => Type::Array,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Subtraction { left, right, .. } => match (self.get_type(left.as_ref(), definitions), self.get_type(right.as_ref(), definitions)) {
                    (Type::Float, Type::Int | Type::Float) => Type::Float,
                    (Type::Int, Type::Float) => Type::Float,
                    (Type::Int, Type::Int) => Type::Int,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Multiplication { left, right, .. } => match (self.get_type(left.as_ref(), definitions), self.get_type(right.as_ref(), definitions)) {
                    (Type::Float, Type::Int | Type::Float) => Type::Float,
                    (Type::Int, Type::Float) => Type::Float,
                    (Type::Int, Type::Int) => Type::Int,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Division { left, right, .. } => match (self.get_type(left.as_ref(), definitions), self.get_type(right.as_ref(), definitions)) {
                    (Type::Float | Type::Int, Type::Int | Type::Float) => Type::Union(vec![Type::Float, Type::Int]),
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Modulo { left, percent, right } => match (self.get_type(left.as_ref(), definitions), self.get_type(right.as_ref(), definitions)) {
                    (Type::Float | Type::Int, Type::Int | Type::Float) => Type::Int,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Exponentiation { left, pow, right } => match (self.get_type(left.as_ref(), definitions), self.get_type(right.as_ref(), definitions)) {
                    (Type::Float | Type::Int, Type::Int | Type::Float) => Type::Union(vec![Type::Float, Type::Int]),
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Negative { minus, right } => match self.get_type(right.as_ref(), definitions) {
                    Type::Float => Type::Float,
                    Type::Int => Type::Int,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Positive { plus, right } => match self.get_type(right.as_ref(), definitions) {
                    Type::Float => Type::Float,
                    Type::Int => Type::Int,
                    _ => Type::Error,
                }
                ArithmeticOperationExpression::PreIncrement { increment, right } => match self.get_type(right.as_ref(), definitions) {
                    Type::Float => Type::Float,
                    Type::Int => Type::Int,
                    Type::String => Type::String,
                    _ => Type::Error,
                }
                ArithmeticOperationExpression::PostIncrement { left, increment } => match self.get_type(left.as_ref(), definitions) {
                    Type::Float => Type::Float,
                    Type::Int => Type::Int,
                    Type::String => Type::String,
                    _ => Type::Error,
                }
                ArithmeticOperationExpression::PreDecrement { decrement, right } => match self.get_type(right.as_ref(), definitions) {
                    Type::Float => Type::Float,
                    Type::Int => Type::Int,
                    Type::String => Type::String,
                    _ => Type::Error,
                }
                ArithmeticOperationExpression::PostDecrement { left, decrement } => match self.get_type(left.as_ref(), definitions) {
                    Type::Float => Type::Float,
                    Type::Int => Type::Int,
                    Type::String => Type::String,
                    _ => Type::Error,
                }
            },
            _ => Type::Mixed,
        }
    }

    pub fn resolve_name(&self, name: &ByteString) -> ByteString {
        // If the name is already fully qualified, return as is.
        if name.bytes.starts_with(b"\\") {
            return name.clone();
        }

        if self.is_in_class() && name == self.classish_context() {
            let mut qualified_name = self.namespace.clone();
            qualified_name.extend(b"\\");
            qualified_name.extend(&name.bytes);
            return qualified_name;
        }

        let parts = name.split(|b| *b == b'\\').collect::<Vec<&[u8]>>();
        let first_part = parts.first().unwrap();

        // Check each imported name to see if it ends with the first part of the
        // given identifier. If it does, we can assume you're referencing either
        // an imported namespace or class that has been imported.
        for imported_name in self.imports.iter() {
            if imported_name.ends_with(first_part) {
                let mut qualified_name = imported_name.clone();
                qualified_name.extend(&name.bytes[first_part.len()..]);

                return qualified_name;
            }
        }

        // If we've reached this point, we have a simple name that
        // is not fully qualified and we have not imported it.
        // We can simply prepend the current namespace to it.
        let mut qualified_name = self.namespace.clone();
        qualified_name.extend(b"\\");
        qualified_name.extend(&name.bytes);

        qualified_name
    }

    pub fn set_namespace(&mut self, namespace: ByteString) {
        self.namespace = namespace;
    }

    pub fn add_import(&mut self, import: ByteString) {
        self.imports.push(import);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}