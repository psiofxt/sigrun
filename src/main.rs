mod wallets;
use wallets::BitcoinWallet;

fn main() {
    BitcoinWallet::from_seed("lol".as_bytes());
}
