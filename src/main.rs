use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(about = "Analyse a file.")]
    Analyse {
        #[clap(help = "The file to analyse.")]
        file: String,
    },
}

fn main() {
    let arguments = Arguments::parse();
}
