use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::Network;
use bitcoin::util::address::Address;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::key::PrivateKey;
use bitcoin::util::bip32::DerivationPath;
use std::str::FromStr;

pub fn generate_address() {
    let secp = Secp256k1::new();
    let sk = ExtendedPrivKey::new_master(Network::Testnet, &[0u8; 32]).unwrap();
    let derived = sk.derive_priv(&secp, &DerivationPath::from_str("m/84'/1'/0'/0/0").unwrap()).unwrap();
    let pk = PrivateKey::new(derived.private_key.key, Network::Testnet);
    let address = Address::p2wpkh(&pk.public_key(&secp), Network::Testnet).unwrap();
    println!("Generated Testnet Address: {}", address);
}

pub fn fetch_utxos(address: &str) {
    let url = format!("https://blockstream.info/testnet/api/address/{}/utxo", address);
    let resp = reqwest::blocking::get(&url).unwrap().text().unwrap();
    println!("UTXOs for {}: {}", address, resp);
}