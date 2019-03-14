#[macro_use] extern crate bacon;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bincode;
use bacon::{ Bacon, BaconState, ciphers::{ Cipher, chacha20::ChaCha20, Decrypt, Encrypt } };
use bigint::uint::U256;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
enum Sex { HotFemale, HotterFemale, Chad }
#[derive(Debug, Deserialize, Serialize)]
struct Dancer {
    name: String,
    favorite_dance: String,
    age: u8,
    sex: Sex,
    address: String
}

// example:
// $ cargo run --example chacha20 102853573294759285723534561345875635123503952762319857163587163501983275012378
fn main() {
    let dancer = Dancer {
        name: "SriChaCa Dunzapawny".to_string(),
        favorite_dance: "Two-Step".to_string(),
        age: 18,
        sex: Sex::HotterFemale,
        address: "7 Park Ave, Olympus Mons, Mars".to_string(),
    };
    // optional description
    let mut bacon_descr = HashMap::new();
    bacon_descr.insert("Type".to_string(), "examples/chacha20.rs::Dancer".to_string());
    bacon_descr.insert("Cipher".to_string(), "bacon::ciphers::chacha20::ChaCha20".to_string());
    let mut bacon = Bacon::new(BaconState::Unfried, Some(bacon_descr), dancer);
    println!("unfried bacon: {:#?}", bacon);

    // receiving 79 decimal string secret from command line
    let  mut args: Vec<String> = std::env::args().collect();
    // create Cipher with cipher specified key length (here: u128)
    let cipher: ChaCha20 = ChaCha20::new(U256::from_dec_str(&args[1]).unwrap(), None);

    // fry bacon
    bacon = cipher.encrypt(bacon);
    println!("fried bacon: {:#?}", bacon);
    // unfry bacon
    let cipher_2: ChaCha20 = ChaCha20::new(U256::from_dec_str(&args[1]).unwrap(), None);

    bacon = cipher_2.decrypt(bacon);
    println!("unfried bacon: {:#?}", bacon);
    let new_dancer: Dancer = unfry!(bacon, Dancer).unwrap();
    dbg!(new_dancer);
}