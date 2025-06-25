use std::io::Read;
use std::error::Error;
use reqwest::blocking::get;
use crate::types::*;

// fetches bundles from arweave based on tx id
pub fn fetch_bundle(tx_id: &str) -> Result<Bundle, Box<dyn Error>> {
    let url = format!("https://arweave.net/{tx_id}");
    println!("fetching bundle... {url}");
    let resp = get(&url)?;
    if !resp.status().is_success() {
        return Err(format!(
            "bundle fetch failed. status and url: {} {}",
            resp.status(),
            resp.url(),
        ).into());
    }
    let body = resp.bytes()?;
    if body.is_empty() {
        return Err("empty bundle".to_string().into());
    }
    // placeholder header for parse.rs to fill
    let header = BundleHeader {
        items: 0,
        entries: vec![],
    };
    let bundle = Bundle {
        header,
        data_items: body.to_vec(),
    };

    Ok(bundle)
}
