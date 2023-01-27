use std::fs::read;

use prettytable::{Table, row};
use statan::{analyser::Analyser, definitions::collector::DefinitionCollector, rules};

use crate::AnalyseCommand;

pub fn run(args: AnalyseCommand) {
    let files = discoverer::discover(&["php"], &["."]).unwrap();
    let mut collector = DefinitionCollector::new();

    for file in files {
        let contents = std::fs::read(&file).unwrap();
        let parse_result = pxp_parser::parse(&contents);
        if parse_result.is_err() {
            continue;
        }
        let mut ast = parse_result.unwrap();
        collector.scan(&mut ast);
    }

    let collection = collector.collect();

    let mut analyser = Analyser::new(collection);
    analyser.add_rule(Box::new(rules::functions::valid_function::ValidFunctionRule));

    let contents = read(&args.file).unwrap();
    let messages = analyser.analyse(args.file, &contents);

    let mut table = Table::new();
    table.add_row(row![messages.get_file()]);
    for message in messages.iter() {
        table.add_row(row![message]);
    }
    table.printstd();
}
