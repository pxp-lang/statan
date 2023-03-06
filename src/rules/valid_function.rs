use pxp_parser::{
    downcast::downcast,
    lexer::byte_string::ByteString,
    node::Node,
    parser::ast::{
        arguments::{Argument, NamedArgument, PositionalArgument},
        identifiers::{Identifier, SimpleIdentifier},
        Expression, FunctionCallExpression,
    },
};

use crate::{
    analyser::{context::Context, messages::MessageCollector},
    definitions::collection::DefinitionCollection,
    rules::Rule,
};

#[derive(Debug)]
pub struct ValidFunctionRule;

impl Rule for ValidFunctionRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<FunctionCallExpression>(node).is_some()
    }

    fn run(
        &mut self,
        node: &mut dyn Node,
        definitions: &DefinitionCollection,
        messages: &mut MessageCollector,
        context: &mut Context,
    ) {
        let function_call_expression = downcast::<FunctionCallExpression>(node).unwrap();

        let (function_name, span) = match function_call_expression.target.as_ref() {
            Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier {
                value: function_name,
                span,
            })) => (function_name, span),
            _ => return,
        };

        let definition = definitions.get_function(function_name, context);

        if definition.is_none() {
            // TODO: Add a check for execution inside of a `function_exists` call.
            messages.error(
                format!(
                    "Function `{}` (DBG: {}, {}) not found",
                    function_name,
                    context.resolve_name(function_name),
                    {
                        let mut global_name = ByteString::default();
                        global_name.extend(b"\\");
                        global_name.extend(&function_name.bytes);
                        global_name
                    }
                ),
                span.line,
            );

            return;
        }

        let definition = definition.unwrap();
        let min_arity = definition.min_arity();
        let max_arity = definition.max_arity();

        if function_call_expression.arguments.arguments.len() < min_arity {
            messages.error(
                format!(
                    "Function {}() requires {} arguments, {} given",
                    function_name,
                    min_arity,
                    function_call_expression.arguments.arguments.len()
                ),
                span.line,
            );
            return;
        }

        if function_call_expression.arguments.arguments.len() > max_arity {
            messages.error(
                format!(
                    "Function {}() requires {} arguments, {} given",
                    function_name,
                    max_arity,
                    function_call_expression.arguments.arguments.len()
                ),
                span.line,
            );
            return;
        }

        let mut has_encountered_named_argument = false;

        for (position, argument) in function_call_expression.arguments.iter().enumerate() {
            match argument {
                Argument::Positional(PositionalArgument {
                    comments: _,
                    ellipsis: _,
                    value,
                }) => {
                    if has_encountered_named_argument {
                        messages.error(
                            "Positional argument cannot follow named argument",
                            span.line,
                        );
                        continue;
                    }

                    // We've already checked that the number of arguments is within the range of the function's arity,
                    // so we can safely unwrap the parameter.
                    let parameter = definition.get_parameter_by_position(position).unwrap();

                    // If parameter has no type, we can't check it.
                    if parameter.type_.is_none() {
                        continue;
                    }

                    let parameter_type = parameter.type_.as_ref().unwrap();
                    let argument_type = context.get_type(value, definitions);

                    if !parameter_type.compatible(&argument_type) {
                        // Doesn't make sense to zero-index the position, so we add 1.
                        messages.error(format!("Argument {} of type {} is not compatible with parameter {} of type {}", position + 1, argument_type, parameter.name, parameter_type), span.line);
                    }
                }
                Argument::Named(NamedArgument {
                    comments: _,
                    name,
                    colon: _,
                    ellipsis: _,
                    value,
                }) => {
                    has_encountered_named_argument = true;

                    let mut parameter = definition.get_parameter_by_name(&name.value);

                    // TODO: Check if the parameter is a spread parameter.
                    if parameter.is_none() {
                        match definition.get_parameter_by_position(position) {
                            Some(p) => {
                                if p.spread {
                                    parameter = Some(p);
                                } else {
                                    messages.error(format!("Function {function_name}() does not have a parameter named {name}"), span.line);
                                    continue;
                                }
                            }
                            None => {
                                messages.error(format!("Function {function_name}() does not have a parameter named {name}"), span.line);
                                continue;
                            }
                        }
                    }

                    let parameter = parameter.unwrap();

                    // If parameter has no type, we can't check it.
                    if parameter.type_.is_none() {
                        continue;
                    }

                    let parameter_type = parameter.type_.as_ref().unwrap();
                    let argument_type = context.get_type(value, definitions);

                    if !parameter_type.compatible(&argument_type) {
                        messages.error(format!("Argument {} of type {} is not compatible with parameter {} of type {}", name, argument_type, parameter.name, parameter_type), span.line);
                    }
                }
            }
        }
    }
}
