mod wallet;

fn main() {
    match wallet::generate_wallet() {
        Ok((mnemonic, address)) => {
            println!("Mnemonic Phrase: {}", mnemonic);
            println!("Bitcoin Address: {}", address);
        }
        Err(e) => {
            eprintln!("Wallet generation failed: {}", e);
        }
    }
}
