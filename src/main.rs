use clap::{Parser, Subcommand};

mod cmd;

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(about = "Analyse a file.")]
    Analyse(AnalyseCommand),
}

#[derive(Debug, Parser)]
pub struct AnalyseCommand {
    #[clap(help = "The file to analyse.")]
    file: String,
}

fn main() {
    let arguments = Arguments::parse();

    match arguments.command {
        Command::Analyse(args) => cmd::analyse::run(args),
    }
}
