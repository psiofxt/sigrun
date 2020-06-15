extern crate bitcoin;
extern crate secp256k1;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;

use hdwallet::{KeyChain, DefaultKeyChain, ExtendedPrivKey};
use secp256k1::Secp256k1;

pub struct BitcoinWallet {
    pub public_key: key::PublicKey,
    private_key: key::PrivateKey,
    change_byte: u32,
    chain_byte: u32,
}

impl BitcoinWallet {
    pub fn from_seed(seed: &[u8]) {
        let master_key = ExtendedPrivKey::with_seed(seed).unwrap();
        let key_chain = DefaultKeyChain::new(master_key);

        let path = format!("m/44'/0'/0'/0/{}", 0);
        let (child_key, _) = key_chain.derive_private_key(path.into()).unwrap();

        let secp = Secp256k1::new();
        let private_key = key::PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: child_key.private_key
        };
        let public_key = key::PublicKey::from_private_key(&secp, &private_key);

        let address_pkh = Address::p2pkh(&public_key, Network::Bitcoin);
        let address_shwpkh = Address::p2shwpkh(&public_key, Network::Bitcoin);
        let address_wpkh = Address::p2wpkh(&public_key, Network::Bitcoin);
        println!("{:?}", address_pkh);
        println!("{:?}", address_shwpkh);
        println!("{:?}\n", address_wpkh);
    }
}
