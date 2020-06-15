extern crate bitcoin;
extern crate secp256k1;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;

use hdwallet::{KeyChain, DefaultKeyChain, ExtendedPrivKey};
use secp256k1::Secp256k1;

mod wallets;
use wallets::BitcoinWallet;

fn main() {
    BitcoinWallet::from_seed("lol".as_bytes());
}
