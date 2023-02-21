use pxp_parser::{node::Node, parser::ast::{StaticMethodCallExpression, Expression, identifiers::{Identifier, SimpleIdentifier}}, downcast::downcast, lexer::byte_string::ByteString};

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
        let class = match definitions.get_class(class_name, context) {
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

            // If the current class is the same as the class that the method is being called on, then we're good.
            if current_class == class && ! has_inherited {
                return;
            }

            // If we're not in the same class, or if the method is inherited, then calling a private method is disallowed.
            if method.is_private() {
                messages.error(format!("Call to private method {class_name}::{method_name}()"), static_method_call.double_colon.line);
                return;
            }

            // If the method is protected, then we need to check if the current class inherits the method from a class in the inheritance chain.
            if current_class.get_inherited_method(method_name, definitions, context).is_none() {
                messages.error(format!("Call to protected method {class_name}::{method_name}()"), static_method_call.double_colon.line);
            }
        }
    }
}