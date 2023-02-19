use std::fmt::Debug;

use pxp_parser::node::Node;

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

pub mod valid_function;
pub mod valid_class;
pub mod valid_static_call;
pub mod void_assignment;
pub mod dump_type;
pub mod valid_this_call;

pub trait Rule: Debug {
    fn should_run(&self, node: &dyn Node) -> bool;
    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context);
}
