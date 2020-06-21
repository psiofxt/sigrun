extern crate bitcoin;
extern crate secp256k1;
extern crate rand;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;

use hdwallet::{KeyChain, DefaultKeyChain, ExtendedPrivKey};
use secp256k1::Secp256k1;
use rand::rngs::OsRng;
use rand::RngCore;

pub struct BitcoinWallet {
    pub public_key: key::PublicKey,
    private_key: key::PrivateKey,
    change_byte: u32,
    chain_byte: u32,
}

impl BitcoinWallet {
    pub fn from_seed(seed: &[u8]) -> (Address, Address, Address) {
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

        (address_pkh, address_shwpkh, address_wpkh)
    }

    /// Naive implementation to match for a lowercase string against wrapped segwit
    pub fn vanity(str_to_match: &str) {
        loop {
            let mut key = [0u8; 128];
            let mut rng = OsRng::new().unwrap();
            rng.fill_bytes(&mut key);
            let (_, address_shwpkh, _) = BitcoinWallet::from_seed(&key);
            if address_shwpkh.to_string().to_lowercase().contains(str_to_match) {
                println!("{:?}", address_shwpkh);
            }
        }
    }
}
