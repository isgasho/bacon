#[macro_use] extern crate bacon;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bincode;

use bacon::{ Bacon, BaconState, ciphers::{ Cipher, speck::Speck, Decrypt, Encrypt, Nonce } };
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
enum Sex { Male, Female, Diverse }
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    sex: Sex,
    address: String,
    descr: String,
}

fn main() {
    let person = Person {
        name: "Ernst Stavro Blofeld".to_string(),
        age: 77,
        sex: Sex::Male,
        address: "In a Vulcano in or near Japan".to_string(),
        descr: "CEO of SPECKTRE. Very evil.".to_string()
    };
    // optional description
    let mut descr: HashMap<String, String> = HashMap::new();
    descr.insert("Cipher".to_string(), "bacon::ciphers::speck::Speck".to_string());
    descr.insert("Type_Definition".to_string(), "bacon::examples::Person { String, u8, bacon::examples::Sex, String, String }".to_string());

    let mut bacon = Bacon::new(BaconState::Unfried, Some(descr), person);
    println!("unfried bacon: {:#?}", bacon);

    // receiving 16 character secret from command line
    let  mut args: Vec<String> = std::env::args().collect();
    // turning key into a u128
    let mut key_u128 = bacon::key_128(&args[1]);
    // create Cipher with cipher specified key length (here: u128)
    let cipher: Speck = Speck::new(key_u128, Nonce::None);
    // empty nullify and drop key as soon as possible
    key_u128 = 0;
    drop(key_u128);

    // fry bacon
    bacon = cipher.encrypt(bacon);
    println!("fried bacon: {:#?}", bacon);
    // unfry bacon
    bacon = cipher.decrypt(bacon);
    println!("unfried bacon: {:#?}", bacon);
    let unkn_person: Person = unfry!(bacon, Person).unwrap();
    dbg!(unkn_person);
}