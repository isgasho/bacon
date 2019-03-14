#[macro_use] extern crate bacon;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bincode;
use bacon::{ Bacon, BaconState, ciphers::{ Cipher, speck::Speck, Decrypt, Encrypt } };
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
    bacon_descr.insert("Type".to_string(), "examples/speck::Dancer".to_string());
    bacon_descr.insert("Cipher".to_string(), "bacon::ciphers::speck::Speck".to_string());
    let mut bacon = Bacon::new(BaconState::Unfried, Some(bacon_descr), dancer);
    println!("unfried bacon: {:#?}", bacon);

    // receiving 16 character secret from command line
    let  mut args: Vec<String> = std::env::args().collect();
    // turning key into a u128
    let mut key_u128 = bacon::key_128(&args[1]);
    // create Cipher with cipher specified key length (here: u128)
    let cipher: Speck = Speck::new(key_u128);
    // empty nullify and drop key as soon as possible
    key_u128 = 0;
    drop(key_u128);

    // fry bacon
    bacon = cipher.encrypt(bacon);
    println!("fried bacon: {:#?}", bacon);
    // unfry bacon
    bacon = cipher.decrypt(bacon);
    println!("unfried bacon: {:#?}", bacon);
    let new_dancer: Dancer = unfry!(bacon, Dancer).unwrap();
    dbg!(new_dancer);
}