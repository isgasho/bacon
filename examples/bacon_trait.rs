/// currently unused


#[forbid(unsafe_code)]
#[macro_use]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use bacon::{ Bacon, Encrypt };
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

// encrypts a struct using the speck algorithm and decrypts it back
// $ cargo run --example bacon { optional 16 character pass } 
fn main() {
    // key
    let args: Vec<String> = std::env::args().collect();
    let mut key_str: String = "".to_string();
    if args.len() > 1 {
        key_str = args[1].clone();
        drop(args);
    } else {
        let mut rng = rand::thread_rng();
        key_str = rng.sample_iter(&Alphanumeric).take(16).collect();
    }
    let key_128 = bacon::key_128(&key_str);
    key_str = "".to_string();
    drop(key_str);
    
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
    let bacon: Bacon = Bacon::fry(vip, key_128);
    dbg!(bacon);
}