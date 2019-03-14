#[macro_use] extern crate bacon;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bincode;
use bacon::{ Bacon, BaconState, ciphers::{ Cipher, speck::Speck, Decrypt, Encrypt } };
use std::collections::HashMap;

#[derive(Serialize)]
enum Sex { HotFemale, HotterFemale, Chad }
#[derive(Serialize)]
struct Dancer {
    name: String,
    favorite_dance: String,
    age: u8,
    sex: Sex
}

fn main() -> Result<(), &'static str> {
    let  mut args: Vec<String> = std::env::args().collect();

    let mut key_128 = bacon::key_128(&args[1]);
    let dancer = Dancer {
        name: "SriChaCa Dunzapawn".to_string(),
        favorite_dance: "Two-Step".to_string(),
        age: 18,
        sex: Sex::HotterFemale
    };
    let mut bacon_descr = HashMap::new();
    bacon_descr.insert("Type".to_string(), "examples/speck::Dancer".to_string());
    bacon_descr.insert("Cipher".to_string(), "bacon::ciphers::speck::Speck".to_string());

    let mut bacon = Bacon::new(BaconState::Unfried, Some(bacon_descr), dancer);
    dbg!(&bacon);
    let cipher: Speck = Speck::new(key_128);
    bacon = cipher.encrypt(bacon);
    dbg!(&bacon);
    bacon = cipher.decrypt(bacon);
    dbg!(&bacon);
    Ok(())
}