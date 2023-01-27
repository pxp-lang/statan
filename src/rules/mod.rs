use std::fmt::Debug;

use pxp_parser::node::Node;

use crate::{definitions::collection::DefinitionCollection, analyser::{messages::MessageCollector, context::Context}};

pub mod functions;

pub trait Rule: Debug {
    fn should_run(&self, node: &dyn Node) -> bool;
    fn run(&mut self, node: &mut dyn Node, definitions: &DefinitionCollection, messages: &mut MessageCollector, context: &mut Context);
}
