#[forbid(unsafe_code)]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use bacon::{ Bacon, Fry, Unfry };
use rand::{ distributions::{ Alphanumeric }, Rng };

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Gender { Female, Male, Undefined }
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
    address: String,
    description: String
}

impl Person {
    fn new() -> Person {
        Person {
            name: String::new(),
            age: 0,
            gender: Gender::Undefined,
            address: String::new(),
            description: String::new()
        }
    }
}
// encrypts a struct using the speck algorithm and decrypts it back
// $ cargo run --example bacon { optional 16 character pass } 
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
    let bacon: Bacon = Bacon::fry(vip, key_128);
    dbg!(&bacon);
    println!();
    // unfry -- not a good solution
    let mut person = Person::new();
    person = Bacon::unfry(bacon, person, key_128).unwrap();
    dbg!(person);

}