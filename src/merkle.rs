use bitcoin_hashes::sha256d::Hash as Sha256dHash;
use bitcoin_hashes::Hash;
use serde::Deserialize;
use reqwest;
use hex;

const TESTNET_API: &str = "https://blockstream.info/testnet/api";

#[derive(Deserialize)]
struct MerkleProof {
    merkle: Vec<String>,
    pos: u32,
}

pub fn verify_merkle(txid: &str, block_hash: &str) {
    let proof_url = format!("{}/tx/{}/merkle-proof", TESTNET_API, txid);

    let response = match reqwest::blocking::get(&proof_url) {
        Ok(resp) => resp,
        Err(_) => {
            println!("❌ Failed to fetch proof from Blockstream. Check your internet or txid.");
            return;
        }
    };

    let proof: MerkleProof = match response.json() {
        Ok(p) => p,
        Err(_) => {
            println!("❌ Failed to parse Merkle proof JSON. The txid might not exist.");
            return;
        }
    };

    let mut h = hex::decode(txid).expect("Invalid txid hex");

    for (i, sibling_hex) in proof.merkle.iter().enumerate() {
        let sibling = hex::decode(sibling_hex).expect("Invalid merkle hash");
        let pair = if (proof.pos >> i) & 1 == 1 {
            [sibling, h.clone()].concat()
        } else {
            [h.clone(), sibling].concat()
        };

        h = Sha256dHash::hash(&pair).to_byte_array().to_vec(); // <-- Fixed: Convert [u8; 32] to Vec<u8>
    }

    println!("✅ Computed Merkle root: {}", hex::encode(h));
    println!("(Compare it manually to the block's merkle root if needed)");
}
