use std::fs::{read, metadata};

use indicatif::ProgressBar;
use prettytable::{Table, row};
use statan::{analyser::Analyser, definitions::collector::DefinitionCollector, rules};
use colored::*;

use crate::AnalyseCommand;

pub fn run(args: AnalyseCommand) {
    let files = discoverer::discover(&["php"], &["."]).unwrap();
    let mut collector = DefinitionCollector::new();

    println!("{}", "> Discovering project definitions...".yellow());

    for file in files {
        let contents = std::fs::read(&file).unwrap();
        let parse_result = pxp_parser::parse(&contents);
        if parse_result.is_err() {
            println!("failed to parse {}", &file.to_str().unwrap());
            println!("{}", parse_result.err().unwrap());
            continue;
        }
        let mut ast = parse_result.unwrap();
        collector.scan(&mut ast);
    }

    println!("{}", "> Analysing project...".yellow());

    let collection = collector.collect();

    let mut analyser = Analyser::new(collection);
    analyser.add_rule(Box::new(rules::valid_assignment::ValidAssignmentRule));
    analyser.add_rule(Box::new(rules::dump_type::DumpTypeRule));
    analyser.add_rule(Box::new(rules::valid_function::ValidFunctionRule));
    analyser.add_rule(Box::new(rules::valid_class::ValidClassRule));
    analyser.add_rule(Box::new(rules::valid_static_call::ValidStaticCallRule));
    analyser.add_rule(Box::new(rules::valid_this_call::ValidThisCallRule));

    let mut message_collections = Vec::new();
    let metadata = metadata(&args.file).unwrap();

    if metadata.is_dir() {
        let files = discoverer::discover(&["php"], &[&args.file]).unwrap();
        let progress_bar = ProgressBar::new(files.len() as u64);
        for file in files {
            let contents = read(&file).unwrap();
            let messages = analyser.analyse(file.to_str().unwrap().to_string(), &contents);
            message_collections.push(messages);
            progress_bar.inc(1);
        }
        progress_bar.finish();
    } else {
        let contents = read(&args.file).unwrap();
        let messages = analyser.analyse(args.file, &contents);
        message_collections.push(messages);
    }

    for messages in message_collections {
        if messages.iter().len() == 0 {
            return;
        }

        let mut table = Table::new();
        table.add_row(row!["Line", messages.get_file()]);
        for message in messages.iter() {
            table.add_row(row![message.line, message.message]);
        }
        table.printstd();
    }
}
