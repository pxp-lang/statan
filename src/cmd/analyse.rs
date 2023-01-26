use std::fs::read;

use statan::{analyser::Analyser, definitions::collector::DefinitionCollector};

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

    let contents = read(&args.file).unwrap();
    let messages = analyser.analyse(args.file, &contents);

    dbg!(messages);
}
