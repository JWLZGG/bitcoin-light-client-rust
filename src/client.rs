use bitcoin::{block::Header as BlockHeader, consensus::deserialize};
use hex::FromHex;

pub fn parse_block_header(hex_header: &str) {
    let raw = Vec::from_hex(hex_header).expect("Invalid hex string");
    let header: BlockHeader = deserialize(&raw).expect("Failed to deserialize header");

    println!("✅ Parsed Block Header:");
    println!(" - Version: {:?}", header.version);
    println!(" - Previous block hash: {}", header.prev_blockhash);
    println!(" - Merkle root: {}", header.merkle_root);
    println!(" - Time: {}", header.time);
    println!(" - Bits: {}", header.bits);
    println!(" - Nonce: {}", header.nonce);
}
