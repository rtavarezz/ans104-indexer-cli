use serde::Serialize;

// this file is based on ans-104 spec: https://github.com/ArweaveTeam/arweave-standards/blob/master/ans/ANS-104.md
#[derive(Serialize, Debug)]
pub struct BundleHeader {
    pub items: u32, // num of data items in bundle
    pub entries: Vec<DataItemMeta>, // N * 64 bytes (32B size + 32B id)
}

#[derive(Serialize, Debug)]
pub struct DataItemMeta {
    pub size: [u8; 32], // data item size
    pub id: [u8; 32], // hash of data item's signature
}
#[derive(Serialize, Debug)]
pub struct Bundle {
    pub header: BundleHeader, // num items(N) + size/id(64B)
    pub data_items: Vec<u8>, // binary encoded data items in the bundle
}

#[derive(Serialize, Debug)]
pub struct ParsedDataItem {
    pub id: String,
    pub signature_type: u16,
    pub signature: String,
    pub owner: String,
    pub data: Option<String>,
}