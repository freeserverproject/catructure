mod command;
mod model;
mod error;
mod ascii_tree;

use std::process;
use command::StructCheckerCLI;

fn main() {
    let result = StructCheckerCLI::run();

    match result {
        Err(err) => {
            println!("{}", err.to_string());
            process::exit(1);
        },
        Ok(()) => {}
    };
}
