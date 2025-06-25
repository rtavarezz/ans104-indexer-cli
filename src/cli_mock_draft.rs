use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
// define args needed for arweave tx
pub struct Cli {
    pub tx_id: String,

    #[arg(short, long)]
    pub out: Option<String>,
}

// mock parser func for fake tx
pub fn parse_tx_id(tx_id: &str, out_path: Option<&str>) {
    // @todo: safety checks
    if tx_id.is_empty() {
        panic!("tx_id is empty");
    }
    println!("HI! parsing tx from viewblock.io: {tx_id}");

    // @todo: fetch bundle based on tx_id
    let mock_output = format!(
        r#"{{
    "tx_id": "{tx_id}",
    "data": ["mock_item1", "mock_item2", "fees", "datachain", "arweave"]
}}"#
    );

    match out_path {
        Some(path) => {
            std::fs::write(path, &mock_output).expect("Failed to write file");
            println!("Output file saved to: {path}");
        }
        None => {
            println!("Output:\n{mock_output}");
        }
    }
}
