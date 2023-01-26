use pxp_parser::parse;

use crate::{definitions::collection::DefinitionCollection, rules::Rule};

use self::messages::MessageCollector;

pub mod messages;

#[derive(Debug)]
pub struct Analyser {
    rules: Vec<Box<dyn Rule>>,
    definitions: DefinitionCollection,
}

impl Analyser {
    pub fn new(definitions: DefinitionCollection) -> Self {
        Self {
            rules: Vec::new(),
            definitions,
        }
    }

    pub fn analyse(&mut self, file: String, contents: &[u8]) -> MessageCollector {
        let mut message_collector = MessageCollector::new(file);

        let parse_result = parse(contents);
        if let Err(error) = parse_result {
            message_collector.add(error.to_string());
            return message_collector;
        }

        let mut ast = parse_result.unwrap();

        return message_collector;
    }

    pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }
}
