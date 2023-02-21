use pxp_parser::{node::Node, downcast::downcast, parser::ast::{MethodCallExpression, Expression, variables::{Variable, SimpleVariable}, identifiers::{Identifier, SimpleIdentifier}, arguments::{Argument, PositionalArgument, NamedArgument}}, lexer::byte_string::ByteString};

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

use super::Rule;

#[derive(Debug)]
pub struct ValidThisCallRule;

impl Rule for ValidThisCallRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<MethodCallExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let method_call_expression = downcast::<MethodCallExpression>(node).unwrap();

        // 1. Check that the method call is on $this.
        match method_call_expression.target.as_ref() {
            Expression::Variable(Variable::SimpleVariable(SimpleVariable { name, .. })) => {
                if name != &ByteString::from(b"$this") {
                    return;
                }
            },
            _ => return,
        }

        // 2. Check that the method name is not variable.
        let method_name = match method_call_expression.method.as_ref() {
            Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { value, .. })) => value,
            _ => return,
        };

        // 3. Check if currently inside of a classish context.
        // TODO: We should also calling $this->foo() inside of a Closure since it could be bound to an object.
        if ! context.is_in_class() {
            messages.error(format!("Calling $this->{method_name}() outside of class context"), method_call_expression.arrow.line);
            return;
        }

        // 4. Get the current classish context.
        let mut classish_context = context.classish_context();

        // 5. Get the current class definition.
        let class_definition = definitions.get_class(classish_context, context).unwrap();

        // 6. Get the method definition from the class.
        let mut method_definition = class_definition.get_method(method_name, definitions, context);
        let call_magic = &ByteString::from(b"__call");
        let has_call_magic = class_definition.get_method(call_magic, definitions, context).is_some() || class_definition.get_inherited_method(call_magic, definitions, context).is_some();

        // 7. Check that the method exists.
        if method_definition.is_none() {
            if let Some((inherited_method_from, inherited_method)) = class_definition.get_inherited_method(method_name, definitions, context) {
                method_definition = Some(inherited_method);
                classish_context = inherited_method_from;
            } else if ! has_call_magic {
                // TODO: Check if class's docblock has an @method.
                messages.error(format!("Call to undefined method $this->{method_name}() on {classish_context}"), method_call_expression.arrow.line);
                return;
            }
        }

        // TODO: Check if class's docblock has an @method.
        if has_call_magic {
            return;
        }

        let method = method_definition.unwrap();

        if ! method.is_public() {
            // 9. Grab the actual context for the method. If the method was inherited, then
            //    the actual context of the method will be the class where it's defined.
            let method_class_context = definitions.get_class(classish_context, context).unwrap();

            // 10. If the method's class context matches the current class context, then
            //     calling a private or protected method is perfectly fine.
            if class_definition != method_class_context && method.is_private() {
                messages.error(format!("Call to private method $this->{method_name}()"), method_call_expression.arrow.line);
                return;
            }
        }

        let span = method_call_expression.arrow;
        let min_arity = method.min_arity();
        let max_arity = method.max_arity();

        if method_call_expression.arguments.arguments.len() < min_arity {
            messages.error(format!("Method $this->{}() requires {} arguments, {} given", method_name, min_arity, method_call_expression.arguments.arguments.len()), span.line);
            return;
        }

        if method_call_expression.arguments.arguments.len() > max_arity {
            messages.error(format!("Method $this->{}() requires {} arguments, {} given", method_name, max_arity, method_call_expression.arguments.arguments.len()), span.line);
            return;
        }

        let mut has_encountered_named_argument = false;

        for (position, argument) in method_call_expression.arguments.iter().enumerate() {
            match argument {
                Argument::Positional(PositionalArgument { comments: _, ellipsis: _, value }) => {
                    if has_encountered_named_argument {
                        messages.error("Positional argument cannot follow named argument", span.line);
                        continue;
                    }
                    
                    // We've already checked that the number of arguments is within the range of the function's arity,
                    // so we can safely unwrap the parameter.
                    let parameter = method.get_parameter_by_position(position).unwrap();

                    // If parameter has no type, we can't check it.
                    if parameter.type_.is_none() {
                        continue;
                    }

                    let parameter_type = parameter.type_.as_ref().unwrap();
                    let argument_type = context.get_type(value, definitions);

                    if ! parameter_type.compatible(&argument_type) {
                        // Doesn't make sense to zero-index the position, so we add 1.
                        messages.error(format!("Argument {} of type {} is not compatible with parameter {} of type {}", position + 1, argument_type, parameter.name, parameter_type), span.line);
                    }
                },
                Argument::Named(NamedArgument { comments: _, name, colon: _, ellipsis: _, value }) => {
                    has_encountered_named_argument = true;

                    let mut parameter = method.get_parameter_by_name(&name.value);

                    // TODO: Check if the parameter is a spread parameter.
                    if parameter.is_none() {
                        match method.get_parameter_by_position(position) {
                            Some(p) => {
                                if p.spread {
                                    parameter = Some(p);
                                } else {
                                    messages.error(format!("Method $this->{method_name}() does not have a parameter named {name}"), span.line);
                                    continue;
                                }
                            },
                            None => {
                                messages.error(format!("Method $this->{method_name}() does not have a parameter named {name}"), span.line);
                                continue;
                            },
                        }
                    }

                    let parameter = parameter.unwrap();

                    // If parameter has no type, we can't check it.
                    if parameter.type_.is_none() {
                        continue;
                    }

                    let parameter_type = parameter.type_.as_ref().unwrap();
                    let argument_type = context.get_type(value, definitions);

                    if ! parameter_type.compatible(&argument_type) {
                        messages.error(format!("Argument {} of type {} is not compatible with parameter {} of type {}", name, argument_type, parameter.name, parameter_type), span.line);
                    }
                },
            }
        }
    }
}