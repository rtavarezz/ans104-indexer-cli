mod cli_mock_draft;
use crate::cli_mock_draft::parse_tx_id;
use clap::Parser;
use cli_mock_draft::Cli;

fn main() {
    let cli_args = Cli::parse();
    parse_tx_id(&cli_args.tx_id, cli_args.out.as_deref());
}
