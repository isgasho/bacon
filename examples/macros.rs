#[forbid(unsafe_code)]
#[macro_use]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use bacon::{ Bacon, ciphers };  // ciphers to use macros fry! unfry! WHY?
use rand::{ distributions::{ Alphanumeric }, Rng };

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Gender { Female, Male }
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
    address: String,
    description: String
}

/// encrypts a struct using the speck algorithm and decrypts it back using macros fry! and unfry!
/// $ cargo run --example bacon { optional 16 character pass } 
fn main() {
    // key
    let args: Vec<String> = std::env::args().collect();
    let mut key_str = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut rng = rand::thread_rng();
        rng.sample_iter(&Alphanumeric).take(16).collect()
    };
    drop(args);
    let key_128 = bacon::key_128(&key_str);
    key_str = "".to_string();               // emptying and
    drop(key_str);                          // dropping key
    
    // create struct
    println!("Creating a struct");
    let vip = Person {
        name: "Ernst Stavro Blofeld".to_string(),
        age: 77,
        gender: Gender::Male,
        address: "Inside a Vulcano, Japan".to_string(),
        description: "CEO of SPECKTRE aka Bacon Industries".to_string()
    };
    dbg!(&vip);
    println!();

    // fry struct
    let fried_bacon: Bacon = fry!(vip, key_128);
    println!("Encrypted struct \"Fried Bacon\"");
    dbg!(&fried_bacon);
    println!();

    // decrypt attempt with correct key
    println!("Attempt to decrypt with correct key.");
    let mut fried_clone = fried_bacon.clone();
    match unfry!(fried_clone, Person, key_128) {
        Ok(p) => { dbg!(p); },
        Err(e) => { dbg!(e); }
    }
    println!();

    // decrypt attempt with wrong key
    println!("Attempt to decrypt with wrong key.");
    let mut rng = rand::thread_rng();
    let wrong_key = rng.gen_range(u128::min_value(), u128::max_value());
    fried_clone = fried_bacon.clone();
    match unfry!(fried_clone, Person, wrong_key) {
        Ok(p) => { dbg!(p); },
        Err(e) => { dbg!(e); }
    }
    println!(); 
}