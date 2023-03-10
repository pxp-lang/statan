use std::fs::{metadata, read};

use colored::*;
use indicatif::ProgressBar;
use prettytable::{row, Table};
use statan::{analyser::Analyser, definitions::collector::DefinitionCollector, rules};

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

    // std::fs::write("./collection.json", serde_json::to_string_pretty(&collection).unwrap()).unwrap();

    let mut analyser = Analyser::new(collection);
    analyser.add_rule(Box::new(rules::valid_assignment::ValidAssignmentRule));
    analyser.add_rule(Box::new(rules::dump_type::DumpTypeRule));
    analyser.add_rule(Box::new(rules::valid_function::ValidFunctionRule));
    analyser.add_rule(Box::new(rules::valid_class::ValidClassRule));
    analyser.add_rule(Box::new(rules::valid_static_call::ValidStaticCallRule));
    analyser.add_rule(Box::new(rules::valid_this_call::ValidThisCallRule));
    analyser.add_rule(Box::new(
        rules::abstract_method_in_non_abstract_class::AbstractMethodInNonAbstractClassRule,
    ));
    analyser.add_rule(Box::new(
        rules::call_private_through_static::CallPrivateThroughStaticRule,
    ));
    analyser.add_rule(Box::new(rules::function_definition::FunctionDefinitionRule));
    analyser.add_rule(Box::new(
        rules::valid_arithmetic_operation::ValidArithmeticOperationRule,
    ));

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
