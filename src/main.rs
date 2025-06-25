mod parser;
mod types;
mod fetch;

use clap::Parser;
use parser::Cli;
use crate::parser::{parse_bundle};

fn main() {
    let cli_args = Cli::parse();
    if let Err(e) = parse_bundle(&cli_args.tx_id, cli_args.out.as_deref()) {
        eprintln!("Error: {e}");
    }
}
