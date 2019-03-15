#[macro_use] extern crate bacon;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bigint;
extern crate bincode;
use bacon::{ Bacon, BaconState, ciphers::{ Authenticate, Cipher, chacha20::ChaCha20, Nonce } };
use bigint::uint::U256;
use std::collections::HashMap;

// example:
// $ cargo run --example hashing
fn main() {

    let msg = "Hello Friend.".to_string();
    let bacon = Bacon::new(BaconState::Unfried, None, msg);
    // ChaCha20 with n: None uses a random nonce and result in a different hash, Decryption will fail
    let key: U256 = U256::from_dec_str("102853573294759285723534561345875635123503952762319857163587163501983275012378").unwrap();
    // Two ChaCha20 with use a random nonce if none explicity passed.
    let cipher: ChaCha20 = ChaCha20::new(key, Nonce::None);
    let cipher_2: ChaCha20 = ChaCha20::new(key, Nonce::None);
    // The two ciphers will have two different hashes. A remote Cipher will not be able to decrypt the bacon
    dbg!(cipher.hash(&bacon));
    dbg!(cipher_2.hash(&bacon));
}