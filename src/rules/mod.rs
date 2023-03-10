use std::fmt::Debug;

use pxp_parser::node::Node;

use crate::{
    analyser::{context::Context, messages::MessageCollector},
    definitions::collection::DefinitionCollection,
};

pub mod abstract_method_in_non_abstract_class;
pub mod call_private_through_static;
pub mod dump_type;
pub mod function_definition;
pub mod valid_arithmetic_operation;
pub mod valid_assignment;
pub mod valid_class;
pub mod valid_function;
pub mod valid_static_call;
pub mod valid_this_call;

pub trait Rule: Debug {
    fn should_run(&self, node: &dyn Node) -> bool;
    fn run(
        &mut self,
        node: &mut dyn Node,
        definitions: &DefinitionCollection,
        messages: &mut MessageCollector,
        context: &mut Context,
    );
}
