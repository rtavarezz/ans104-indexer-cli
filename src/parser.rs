use clap::Parser;
use std::convert::TryInto;
use std::error::Error;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::fetch::fetch_bundle;
use crate::types::{Bundle, DataItemMeta, BundleHeader, ParsedDataItem};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    pub tx_id: String,

    #[arg(short, long)]
    pub out: Option<String>,
}

pub fn parse_bundle(tx_id: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
    let bundle = fetch_bundle(tx_id)?;
    let parsed = parse_bundle_bytes(&bundle.data_items)?;
    let items = parse_bundle_data_items(&parsed)?;
    let json = serde_json::to_string_pretty(&items)?;

    match output {
        Some(path) => {
            std::fs::write(path, json)?;
            println!("Bundle written to: {}", path);
        }
        None => {
            println!("Parsed Bundle: \n{json}");
        }
    }

    Ok(())

}

pub fn parse_bundle_bytes(data: &[u8]) -> Result<Bundle, Box<dyn Error>> {
    const HEADER_SIZE: usize = 32;
    const ENTRY_SIZE: usize = 64;
    const SIZE: usize = 32;

    if data.len() < 32 {
        return Err("not enough data".into());
    }

    let items = u32::from_le_bytes(data[0..4].try_into()?);
    if items as usize * 64 > data.len() {
        return Err("Bundle header claims more items than data size".into());
    }

    // read entries
    let mut entries = Vec::with_capacity(items as usize);
    let mut offset = HEADER_SIZE;

    for i in 0..items {
        let end = offset + ENTRY_SIZE;
        if end > data.len() {
            return Err(format!("EOF while parsing {i} entry").into());
        }
        let size = data[offset..(offset + SIZE)].try_into()?;
        let id = data[offset + SIZE..end].try_into()?;
        entries.push(DataItemMeta { size, id });
        offset = end;
    }
    let remaining_data = data[offset..].to_vec();
    let bundle = Bundle {
        header: BundleHeader{ items, entries },
        data_items: remaining_data,
    };

    Ok(bundle)
}

pub fn parse_data_item(bytes: &[u8]) -> Result<ParsedDataItem, Box<dyn Error>> {
    let mut offset = 0;

    let signature_type = u16::from_be_bytes(bytes[offset..offset + 2].try_into()?);
    offset += 2;

    let signature = &bytes[offset..offset + 512];
    let signature_hex = hex::encode(signature);
    offset += 512;

    let owner = &bytes[offset..offset + 512];
    let owner_hex = hex::encode(owner);
    offset += 512;

    let target_present = bytes[offset];
    offset += 1;

    if target_present == 1 {
        offset += 32; //skip
    }

    let anchor_present = bytes[offset];
    offset += 1;

    if anchor_present == 1 {
        offset += 32; // skip
    }

    let _num_tags = u64::from_le_bytes(bytes[offset..offset + 8].try_into()?);
    offset += 8;
    let tags_bytes = u64::from_le_bytes(bytes[offset..offset + 8].try_into()?);
    offset += 8;
    offset += tags_bytes as usize;

    let data = if offset < bytes.len() {
        Some(STANDARD.encode(&bytes[offset..]))
    } else {
        None
    };

    let id = hex::encode(sha256::digest(signature));

    Ok(ParsedDataItem {
        id,
        signature_type,
        signature: signature_hex,
        owner: owner_hex,
        data,
    })
}

pub fn parse_bundle_data_items(bundle: &Bundle) -> Result<Vec<ParsedDataItem>, Box<dyn Error>> {
    let mut parsed_items = Vec::with_capacity(bundle.header.items as usize);
    let mut offset = 0;

    for entry in &bundle.header.entries {
        let size = u64::from_le_bytes(entry.size[..8].try_into()?);
        let end = offset + size as usize;

        if end > bundle.data_items.len() {
            return Err(format!("invalid data size at offset {offset}").into());
        }

        let item_bytes = &bundle.data_items[offset..end];
        let parsed = parse_data_item(item_bytes)?;
        parsed_items.push(parsed);

        offset = end;
    }

    Ok(parsed_items)
}