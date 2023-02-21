use pxp_parser::{node::Node, parser::ast::{StaticMethodCallExpression, Expression, identifiers::{Identifier, SimpleIdentifier}, arguments::{Argument, PositionalArgument, NamedArgument}}, downcast::downcast, lexer::byte_string::ByteString};

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

use super::Rule;

#[derive(Debug)]
pub struct ValidStaticCallRule;

impl Rule for ValidStaticCallRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<StaticMethodCallExpression>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let static_method_call = downcast::<StaticMethodCallExpression>(node).unwrap();

        // 1. Check that the method is a simple identifier, i.e. Foo::bar(). 
        let method_name = match &static_method_call.method {
            Identifier::SimpleIdentifier(SimpleIdentifier { value, .. }) => value,
            _ => return,
        };

        // 2. Get the class name based on the left-hand side of the call.
        //    If the method call is on `self`, then the class name is pulled from the context.
        //    If the method call is on `static`, then the class name is pulled from the context.
        //    If the method call is on `parent`, then the class name is pulled from the context.
        let mut class_name = match static_method_call.target.as_ref() {
            Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { value, .. })) => value,
            Expression::Self_ => {
                if ! context.is_in_class() {
                    messages.error(format!("Calling self::{method_name}() outside of class context"), static_method_call.double_colon.line);
                    return;
                }

                context.classish_context()
            },
            Expression::Static => {
                if ! context.is_in_class() {
                    messages.error(format!("Calling static::{method_name}() outside of class context"), static_method_call.double_colon.line);
                    return;
                }

                context.classish_context()
            },
            Expression::Parent => {
                if ! context.is_in_class() {
                    messages.error(format!("Calling parent::{method_name}() outside of class context"), static_method_call.double_colon.line);
                    return;
                }

                let child_class = definitions.get_class(context.classish_context(), context).unwrap();
                
                if child_class.extends.is_none() {
                    messages.error(format!(
                        "Calling parent::{}() but {} does not extend any class",
                        method_name,
                        context.classish_context()
                    ), static_method_call.double_colon.line);
                    return;
                }

                child_class.extends.as_ref().unwrap()
            },
            _ => return,
        };

        // 3. Get the class definition from the definitions collection.
        let mut class = match definitions.get_class(class_name, context) {
            Some(class) => class,
            None => {
                messages.error(format!("Call to {class_name}::{method_name}() on unknown class {class_name}"), static_method_call.double_colon.line);
                return;
            },
        };

        // 4. Get the method definition from the class definition.
        let mut method = class.get_method(method_name, definitions, context);
        let mut has_inherited = false;
        let call_static = &ByteString::from("__callStatic");
        let has_call_static = class.get_method(call_static, definitions, context).is_some() || class.get_inherited_method(call_static, definitions, context).is_some();

        // 5. Check that the method exists.
        if method.is_none() {
            if let Some((inherited_method_from, inherited_method)) = class.get_inherited_method(method_name, definitions, context) {
                method = Some(inherited_method);
                class_name = inherited_method_from;
                class = definitions.get_class(class_name, context).unwrap();
                has_inherited = true;
            } else if ! has_call_static {
                // TODO: Check if class's docblock has an @method.
                messages.error(format!("Call to undefined method {class_name}::{method_name}()"), static_method_call.double_colon.line);
                return;
            }
        }

        // TODO: Check if class's docblock has an @method.
        if has_call_static {
            return;
        }

        let method = method.unwrap();

        // 6. Check that the method is static.
        if ! method.is_static() {
            messages.error(format!("Call to non-static method {class_name}::{method_name}()"), static_method_call.double_colon.line);
            return;
        }

        // 7. Check that the method is not abstract.
        // TODO: We should allow calling an abstract method as long as you're inside an abstract context.
        if method.is_abstract() {
            messages.error(format!("Call to abstract method {class_name}::{method_name}()"), static_method_call.double_colon.line);
            return;
        }

        // 8. If the method is public, return early to avoid unnecessary checks.
        if method.is_protected() || method.is_private() {
            // If we're not in a class context, then calling a static protected or private method isn't allowed at all.
            if ! context.is_in_class() {
                messages.error(format!("Call to {} method {}::{}()", if method.is_protected() { "protected" } else if method.is_private() { "private" } else { unreachable!() }, class_name, method_name), static_method_call.double_colon.line);
                return;
            }

            let current_class = definitions.get_class(context.classish_context(), context).unwrap();

            // If we're not in the same class, or if the method is inherited, then calling a private method is disallowed.
            if current_class != class && has_inherited && method.is_private() {
                messages.error(format!("Call to private method {class_name}::{method_name}()"), static_method_call.double_colon.line);
                return;
            }

            // If the method is protected, then we need to check if the current class inherits the method from a class in the inheritance chain.
            if method.is_protected() && current_class.get_inherited_method(method_name, definitions, context).is_none() {
                messages.error(format!("Call to protected method {class_name}::{method_name}()"), static_method_call.double_colon.line);
                return;
            }
        }

        let span = static_method_call.double_colon;
        let min_arity = method.min_arity();
        let max_arity = method.max_arity();

        if static_method_call.arguments.arguments.len() < min_arity {
            messages.error(format!("Method {class_name}::{method_name}() requires {} arguments, {} given", min_arity, static_method_call.arguments.arguments.len()), span.line);
            return;
        }

        if static_method_call.arguments.arguments.len() > max_arity {
            messages.error(format!("Method {class_name}::{}() requires {} arguments, {} given", method_name, max_arity, static_method_call.arguments.arguments.len()), span.line);
            return;
        }

        let mut has_encountered_named_argument = false;

        for (position, argument) in static_method_call.arguments.iter().enumerate() {
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