use pxp_parser::{node::Node, downcast::downcast, parser::ast::{MethodCallExpression, Expression, variables::{Variable, SimpleVariable}, identifiers::{Identifier, SimpleIdentifier}}, lexer::byte_string::ByteString};

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
            messages.add(format!("Calling $this->{}() outside of class context", method_name));
            return;
        }

        // 4. Get the current classish context.
        let mut classish_context = context.classish_context();

        // 5. Get the current class definition.
        let class_definition = definitions.get_class(classish_context, &context).unwrap();

        // 6. Get the method definition from the class.
        let mut method_definition = class_definition.get_method(method_name, definitions, context);
        let mut has_inherited = false;
        let call_magic = &ByteString::from(b"__call");
        let has_call_magic = class_definition.get_method(call_magic, definitions, context).is_some();

        // 7. Check that the method exists.
        if method_definition.is_none() {
            if let Some((inherited_method_from, inherited_method)) = class_definition.get_inherited_method(method_name, definitions, context) {
                method_definition = Some(inherited_method);
                classish_context = inherited_method_from;
                has_inherited = true;
            } else if ! has_call_magic {
                // TODO: Check if class's docblock has an @method.
                messages.add(format!(
                    "Call to undefined method $this->{}() on {}",
                    method_name,
                    classish_context,
                ));
                return;
            }
        }

        // TODO: Check if class's docblock has an @method.
        if has_call_magic {
            return;
        }

        let method = method_definition.unwrap();

        // 7. Check if method is static.
        // TODO: This is actually valid PHP code, but we should probably warn about it when we can.
        if method.is_static() {
            messages.add(format!(
                "Calling $this->{}() but {} is a static method",
                method_name,
                method_name,
            ));
        }

        // TODO: Do we need some magic logic here for handling abstract calls?

        // 8. If the method is public, return early to avoid unnecessary checks.
        if method.is_public() {
            return;
        }

        // 9. Grab the actual context for the method. If the method was inherited, then
        //    the actual context of the method will be the class where it's defined.
        let method_class_context = definitions.get_class(classish_context, context).unwrap();

        // 10. If the method's class context matches the current class context, then
        //     calling a private or protected method is perfectly fine.
        if class_definition == method_class_context && ! has_inherited {
            return;
        }

        // 11. If the method is private and the contexts do not match, then a private call
        //     is disallowed.
        if method.is_private() {
            messages.add(format!(
                "Call to private method $this->{}()",
                method_name,
            ));
            return;
        }
    }
}