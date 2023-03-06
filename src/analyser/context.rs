use std::collections::HashMap;

use crate::{definitions::collection::DefinitionCollection, shared::types::Type};
use pxp_parser::{
    lexer::byte_string::ByteString,
    parser::ast::{
        functions::{ArrowFunctionExpression, ClosureExpression},
        identifiers::{Identifier, SimpleIdentifier},
        literals::Literal,
        operators::{
            ArithmeticOperationExpression, AssignmentOperationExpression,
            ComparisonOperationExpression,
        },
        variables::{SimpleVariable, Variable},
        CastExpression, CloneExpression, DefaultMatchArm, ErrorSuppressExpression, Expression,
        FunctionCallExpression, MagicConstantExpression, MatchArmBody, MatchExpression,
        NewExpression, ParenthesizedExpression, ReferenceExpression, ShortMatchExpression,
    },
};

#[derive(Debug, Clone)]
pub struct Context {
    namespace: ByteString,
    imports: Vec<ByteString>,
    variables: HashMap<ByteString, Type>,
    classish_context: Option<ByteString>,
    function_context: Option<ByteString>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            namespace: ByteString::default(),
            imports: Vec::new(),
            variables: HashMap::new(),
            classish_context: None,
            function_context: None,
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
            function_context: self.function_context.clone(),
        }
    }

    pub fn set_function_context(&mut self, name: &ByteString) {
        self.function_context = Some(name.clone());
    }

    pub fn is_in_function(&self) -> bool {
        self.function_context.is_some()
    }

    pub fn function_context(&self) -> &ByteString {
        self.function_context.as_ref().unwrap()
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
            }
            Expression::FunctionCall(FunctionCallExpression { target, .. }) => {
                match target.as_ref() {
                    Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier {
                        value: function_name,
                        ..
                    })) => {
                        if let Some(function_definition) =
                            definitions.get_function(function_name, self)
                        {
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
                    }
                    _ => Type::Mixed,
                }
            }
            Expression::New(NewExpression { target, .. }) => match target.as_ref() {
                Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier {
                    value,
                    ..
                })) => {
                    let class = definitions.get_class(value, self).unwrap();
                    Type::Named(class.name.clone())
                }
                _ => Type::Object,
            },
            Expression::LogicalOperation(_) => Type::Bool,
            Expression::ComparisonOperation(operation) => match operation {
                ComparisonOperationExpression::Spaceship { .. } => Type::Int,
                _ => Type::Bool,
            },
            Expression::ArithmeticOperation(operation) => match operation {
                ArithmeticOperationExpression::Addition { left, right, .. } => match (
                    self.get_type(left.as_ref(), definitions),
                    self.get_type(right.as_ref(), definitions),
                ) {
                    (Type::Float, Type::Int | Type::Float) => Type::Float,
                    (Type::Int, Type::Float) => Type::Float,
                    (Type::Int, Type::Int) => Type::Int,
                    (Type::Array, Type::Array) => Type::Array,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Subtraction { left, right, .. } => match (
                    self.get_type(left.as_ref(), definitions),
                    self.get_type(right.as_ref(), definitions),
                ) {
                    (Type::Float, Type::Int | Type::Float) => Type::Float,
                    (Type::Int, Type::Float) => Type::Float,
                    (Type::Int, Type::Int) => Type::Int,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Multiplication { left, right, .. } => match (
                    self.get_type(left.as_ref(), definitions),
                    self.get_type(right.as_ref(), definitions),
                ) {
                    (Type::Float, Type::Int | Type::Float) => Type::Float,
                    (Type::Int, Type::Float) => Type::Float,
                    (Type::Int, Type::Int) => Type::Int,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Division { left, right, .. } => match (
                    self.get_type(left.as_ref(), definitions),
                    self.get_type(right.as_ref(), definitions),
                ) {
                    (Type::Float | Type::Int, Type::Int | Type::Float) => {
                        Type::Union(vec![Type::Float, Type::Int])
                    }
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Modulo { left, right, .. } => match (
                    self.get_type(left.as_ref(), definitions),
                    self.get_type(right.as_ref(), definitions),
                ) {
                    (Type::Float | Type::Int, Type::Int | Type::Float) => Type::Int,
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Exponentiation { left, right, .. } => match (
                    self.get_type(left.as_ref(), definitions),
                    self.get_type(right.as_ref(), definitions),
                ) {
                    (Type::Float | Type::Int, Type::Int | Type::Float) => {
                        Type::Union(vec![Type::Float, Type::Int])
                    }
                    _ => Type::Error,
                },
                ArithmeticOperationExpression::Negative { right, .. } => {
                    match self.get_type(right.as_ref(), definitions) {
                        Type::Float => Type::Float,
                        Type::Int => Type::Int,
                        _ => Type::Error,
                    }
                }
                ArithmeticOperationExpression::Positive { right, .. } => {
                    match self.get_type(right.as_ref(), definitions) {
                        Type::Float => Type::Float,
                        Type::Int => Type::Int,
                        _ => Type::Error,
                    }
                }
                ArithmeticOperationExpression::PreIncrement { right, .. } => {
                    match self.get_type(right.as_ref(), definitions) {
                        Type::Float => Type::Float,
                        Type::Int => Type::Int,
                        Type::String => Type::String,
                        _ => Type::Error,
                    }
                }
                ArithmeticOperationExpression::PostIncrement { left, .. } => {
                    match self.get_type(left.as_ref(), definitions) {
                        Type::Float => Type::Float,
                        Type::Int => Type::Int,
                        Type::String => Type::String,
                        _ => Type::Error,
                    }
                }
                ArithmeticOperationExpression::PreDecrement { right, .. } => {
                    match self.get_type(right.as_ref(), definitions) {
                        Type::Float => Type::Float,
                        Type::Int => Type::Int,
                        Type::String => Type::String,
                        _ => Type::Error,
                    }
                }
                ArithmeticOperationExpression::PostDecrement { left, .. } => {
                    match self.get_type(left.as_ref(), definitions) {
                        Type::Float => Type::Float,
                        Type::Int => Type::Int,
                        Type::String => Type::String,
                        _ => Type::Error,
                    }
                }
            },
            Expression::Die(_) | Expression::Exit(_) => Type::Never,
            Expression::Eval(_) => Type::Mixed,
            Expression::Empty(_) | Expression::Isset(_) => Type::Bool,
            Expression::Unset(_) => Type::Void,
            Expression::Print(_) => Type::Int,
            Expression::ErrorSuppress(ErrorSuppressExpression { expr, .. }) => {
                self.get_type(expr, definitions)
            }
            Expression::Parenthesized(ParenthesizedExpression { expr, .. }) => {
                self.get_type(expr, definitions)
            }
            Expression::Include(_)
            | Expression::IncludeOnce(_)
            | Expression::Require(_)
            | Expression::RequireOnce(_) => Type::Mixed,
            Expression::AssignmentOperation(operation) => match operation {
                AssignmentOperationExpression::Assign { right, .. } => {
                    self.get_type(right.as_ref(), definitions)
                }
                AssignmentOperationExpression::Addition {
                    left,
                    plus_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::Subtraction {
                    left,
                    minus_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::Multiplication {
                    left,
                    asterisk_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::Division {
                    left,
                    slash_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::Modulo {
                    left,
                    percent_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::Exponentiation {
                    left,
                    pow_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::Concat {
                    left,
                    dot_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::BitwiseAnd {
                    left,
                    ampersand_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::BitwiseOr {
                    left,
                    pipe_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::BitwiseXor {
                    left,
                    caret_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::LeftShift {
                    left,
                    left_shift_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::RightShift {
                    left,
                    right_shift_equals,
                    right,
                } => todo!(),
                AssignmentOperationExpression::Coalesce {
                    left,
                    coalesce_equals,
                    right,
                } => todo!(),
            },
            Expression::BitwiseOperation(_) => todo!(),
            Expression::RangeOperation(_) => todo!(),
            Expression::Concat(_) => Type::String,
            Expression::Instanceof(_) => Type::Bool,
            Expression::Reference(ReferenceExpression { right, .. }) => {
                self.get_type(right, definitions)
            }
            Expression::Identifier(_) => unreachable!(),
            Expression::FunctionClosureCreation(_) => Type::Callable,
            Expression::MethodCall(_) => todo!(),
            Expression::MethodClosureCreation(_) => todo!(),
            Expression::NullsafeMethodCall(_) => todo!(),
            Expression::StaticMethodCall(_) => todo!(),
            Expression::StaticVariableMethodCall(_) => todo!(),
            Expression::StaticMethodClosureCreation(_) => todo!(),
            Expression::StaticVariableMethodClosureCreation(_) => todo!(),
            Expression::PropertyFetch(_) => todo!(),
            Expression::NullsafePropertyFetch(_) => todo!(),
            Expression::StaticPropertyFetch(_) => todo!(),
            Expression::ConstantFetch(_) => Type::Mixed,
            Expression::Static => unreachable!(),
            Expression::Self_ => unreachable!(),
            Expression::Parent => unreachable!(),
            Expression::ShortArray(_) => Type::Array,
            Expression::Array(_) => Type::Array,
            Expression::List(_) => unreachable!(),
            Expression::Closure(ClosureExpression { return_type, .. }) => return_type
                .as_ref()
                .map(|t| Type::from(&t.data_type))
                .unwrap_or(Type::Mixed),
            Expression::ArrowFunction(ArrowFunctionExpression { return_type, .. }) => return_type
                .as_ref()
                .map(|t| Type::from(&t.data_type))
                .unwrap_or(Type::Mixed),
            Expression::InterpolatedString(_) => Type::String,
            Expression::Heredoc(_) => Type::String,
            Expression::Nowdoc(_) => Type::String,
            Expression::ShellExec(_) => Type::String,
            Expression::AnonymousClass(_) => unreachable!(),
            // TODO: Make this more accurate once we have a better knowledge of
            //       the values stored inside of an array.
            Expression::ArrayIndex(_) => Type::Mixed,
            Expression::MagicConstant(constant) => match constant {
                MagicConstantExpression::Directory(_) => Type::String,
                MagicConstantExpression::File(_) => Type::String,
                MagicConstantExpression::Line(_) => Type::Int,
                MagicConstantExpression::Class(_) => Type::String,
                MagicConstantExpression::Function(_) => Type::String,
                MagicConstantExpression::Method(_) => Type::String,
                MagicConstantExpression::Namespace(_) => Type::String,
                MagicConstantExpression::Trait(_) => Type::String,
                MagicConstantExpression::CompilerHaltOffset(_) => Type::Int,
            },
            Expression::ShortTernary(_) => todo!(),
            Expression::Ternary(_) => todo!(),
            Expression::Coalesce(_) => todo!(),
            Expression::Clone(CloneExpression { target }) => {
                self.get_type(target.as_ref(), definitions)
            }
            Expression::Match(MatchExpression { default, arms, .. }) => {
                let mut types = vec![];

                for arm in arms.iter() {
                    types.push(self.get_type(
                        match &arm.body {
                            MatchArmBody::Block { statements, .. } => todo!(),
                            MatchArmBody::Expression { expression } => &expression,
                        },
                        definitions,
                    ));
                }

                if let Some(default) = default {
                    types.push(self.get_type(
                        match &default.as_ref().body {
                            MatchArmBody::Block { statements, .. } => todo!(),
                            MatchArmBody::Expression { expression } => &expression,
                        },
                        definitions,
                    ));
                }

                Type::Union(types)
            }
            Expression::ShortMatch(ShortMatchExpression { default, arms, .. }) => {
                let mut types = vec![];

                for arm in arms.iter() {
                    types.push(self.get_type(
                        match &arm.body {
                            MatchArmBody::Block { statements, .. } => todo!(),
                            MatchArmBody::Expression { expression } => &expression,
                        },
                        definitions,
                    ));
                }

                if let Some(default) = default {
                    types.push(self.get_type(
                        match &default.as_ref().body {
                            MatchArmBody::Block { statements, .. } => todo!(),
                            MatchArmBody::Expression { expression } => &expression,
                        },
                        definitions,
                    ));
                }

                Type::Union(types)
            }
            Expression::Throw(_) => Type::Never,
            Expression::Yield(_) => todo!(),
            Expression::YieldFrom(_) => todo!(),
            Expression::Cast(CastExpression { kind, value, .. }) => {
                match (kind, self.get_type(value, definitions)) {
                    _ => todo!(),
                }
            }
            Expression::Noop => todo!(),
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
