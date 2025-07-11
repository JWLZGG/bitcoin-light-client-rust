use bitcoin::{
    bip32::{ExtendedPrivKey, DerivationPath},
    Network, Address, secp256k1::Secp256k1,
};
use bip39::{Language, Mnemonic};
use rand::rngs::OsRng;
use rand::RngCore;
use std::str::FromStr;

pub fn generate_wallet() -> Result<(String, Address), Box<dyn std::error::Error>> {
    let mut rng = OsRng;
    let mut entropy = [0u8; 16]; // 128-bit entropy for 12-word mnemonic
    rng.fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy)?;
    let seed = mnemonic.to_seed("");

    let secp = Secp256k1::new();
    let xpriv = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)?;
    let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0")?;
    let child = xpriv.derive_priv(&secp, &derivation_path)?;

    let private_key = child.private_key;
    let secp_pub   = private_key.public_key(&secp);
    let pubkey = bitcoin::PublicKey::new(secp_pub);
    let address = Address::p2wpkh(&pubkey, Network::Bitcoin)?;


    Ok((mnemonic.to_string(), address))
}

