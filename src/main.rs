use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(about = "Analyse a file.")]
    Analyse,
}

fn main() {
    let arguments = Arguments::parse();
}
