use clap::{Parser, Subcommand};

pub mod check;

#[derive(Parser)]
#[command(author, version, about)]
pub struct StructCheckerCLI {
    #[clap(subcommand)]
    pub subcommand: SubCommand
}

impl StructCheckerCLI {
    pub fn run() {
        let cli = StructCheckerCLI::parse();

        match cli.subcommand {
            SubCommand::Check(arg) => check::run(arg)
        }
    }
}

#[derive(Subcommand)]
pub enum SubCommand {
    Check(check::Arg)
}
