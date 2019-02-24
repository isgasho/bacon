#[forbid(unsafe_code)]
#[macro_use]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use bacon::Bacon;
use rand::prelude::*;

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
// $ cargo run --example bacon {16 character pass} 
// TODO: $ cargo run --example bacon {16 character pass} '{ "name": "Rihanna Po Lanna", "age": 32, "gender": "Female", "address": "322 Park Ave, Olympus Mons, Mars".to_string() }'
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let key = bacon::key_128(&args[1]);

// a previously fried Person, password ungfortunately lost
// Bacon {
//     data: [
//         248894775666274688630406846778229178656,
//         11111065244041041936913214710176640135,
//         98935423962589080736130325361634190843,
//         13103328784798148131135224320768214135,
//         97415904987455850552023041001375567129,
//         325247580771718837179507735766775096367,
//         97404749756795619086904136849412589904,
//         82987549362421738383640827970438931211
//     ]
// }

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
    let fried_bacon: Bacon = fry!(vip, key);
    println!("Encrypted struct \"Fried Bacon\"");
    dbg!(&fried_bacon);
    println!();

    // decrypt attempt with wrong key
    println!("Attempt to decrypt with wrong key.");
    let mut rng = rand::thread_rng();
    let wrong_key = rng.gen_range(u128::min_value(), u128::max_value());
    let fried_clone = fried_bacon.clone();
    match unfry!(fried_clone, Person, wrong_key) {
        Ok(p) => { dbg!(p); },
        Err(e) => { dbg!(e); }
    }
    println!();

    // decrypt attempt with correct key
    println!("Attempt to decrypt with correct key.");
    match unfry!(fried_bacon, Person, key) {
        Ok(p) => { dbg!(p); },
        Err(e) => { dbg!(e); }
    }
}