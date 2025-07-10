mod wallet;
mod client;

use wallet::{generate_address, fetch_utxos};
use client::sync_headers;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        return;
    }

    match args[1].as_str() {
        "generate-address" => generate_address(),
        "fetch-utxos" => {
            if args.len() < 3 {
                eprintln!("Usage: fetch-utxos <bitcoin_address>");
            } else {
                fetch_utxos(&args[2]);
            }
        }
        "sync-headers" => sync_headers(),
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}