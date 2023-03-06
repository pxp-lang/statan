use pxp_parser::{node::Node, downcast::downcast, parser::ast::{functions::FunctionStatement, data_type::Type as ParsedType, Expression}};

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

use super::Rule;

#[derive(Debug)]
pub struct FunctionDefinitionRule;

impl Rule for FunctionDefinitionRule {
    fn should_run(&self, node: &dyn Node) -> bool {
        downcast::<FunctionStatement>(node).is_some()
    }

    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context) {
        let function_statement = downcast::<FunctionStatement>(node).unwrap();

        for parameter in function_statement.parameters.iter() {
            // TODO: Validate the type of the parameter.
            match &parameter.data_type {
                Some(ty) => match ty {
                    ParsedType::Void(span) => messages.warning(format!("Parameter {} has invalid type void.", parameter.name), span.line),
                    ParsedType::Never(span) => messages.warning(format!("Parameter {} has invalid type never.", parameter.name), span.line),
                    _ => {},
                },
                None => {
                    messages.warning(format!("Parameter {} has no type.", parameter.name), parameter.name.span.line);
                    continue;
                },
            };
            
            if let Some(Expression::Null) = parameter.default {
                if ! type_is_nullable(parameter.data_type.as_ref().unwrap()) {
                    messages.warning(format!("Parameter {} has a default value of null, but does not have a nullable type.", parameter.name), parameter.name.span.line);
                }
            }
        }

        if function_statement.return_type.is_none() {
            messages.warning(format!("Function {} has no return type.", function_statement.name), function_statement.name.span.line);
        }
    }
}

fn type_is_nullable(ty: &ParsedType) -> bool {
    match ty {
        ParsedType::Union(tys) => tys.iter().any(|ty| type_is_nullable(ty)),
        ParsedType::Null(_) => true,
        ParsedType::Nullable(..) => true,
        _ => false,
    }
}