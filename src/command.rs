use clap::{Parser, Subcommand};

use crate::error::Result;

pub mod check;

#[derive(Parser)]
#[command(author, version, about)]
pub struct StructCheckerCLI {
    #[clap(subcommand)]
    pub subcommand: SubCommand
}

impl StructCheckerCLI {
    pub fn run() -> Result<()> {
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
