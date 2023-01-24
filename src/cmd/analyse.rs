use statan::definitions::collector::DefinitionCollector;

use crate::AnalyseCommand;

pub fn run(command: AnalyseCommand) {
    let files = discoverer::discover(&["php"], &["."]).unwrap();
    let mut collector = DefinitionCollector::new();

    for file in files {
        let contents = std::fs::read(&file).unwrap();
        let mut ast = pxp_parser::parse(&contents).unwrap();
        
        collector.scan(&mut ast);
    }
}
