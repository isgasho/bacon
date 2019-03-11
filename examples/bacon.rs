#[forbid(unsafe_code)]
#[macro_use]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use bacon::{ Bacon, speck };  // speck to use macros fry! unfry!
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

/* a previously fried Person, password ungfortunately lost
    fn test_fried_bacon() -> Bacon {
        let ukwn_fried_person = "{
            \"data\": [
                199989347279576730303273302831594482298,
                89686176579382696547722799744440123650,
                200571766683922312051532064628431861624,
                272773095845529367494637394170706300494,
                135123051543652942388160897815955354259,
                241581523358705025852766085352301738865,
                294742954104957886459273945034249829905,
                170057818899735271015144833461180432525,
                86971309882052014176151965736908291274
            ]
        }";
        let person: Bacon = serde_json::from_str(ukwn_fried_person).unwrap();
        person
    }
*/
// encrypts a struct using the speck algorithm and decrypts it back using macros fry! and unfry!
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