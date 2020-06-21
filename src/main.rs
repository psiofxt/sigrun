mod wallets;
use wallets::BitcoinWallet;

fn main() {
    let (address_pkh, address_shwpkh, address_wpkh) = BitcoinWallet::from_seed("lol".as_bytes());
    println!("{:?}", address_pkh);
    println!("{:?}", address_shwpkh);
    println!("{:?}\n", address_wpkh);
    
    BitcoinWallet::vanity("lol");
}
