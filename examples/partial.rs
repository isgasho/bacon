#[forbid(unsafe_code)]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use bacon::{ Bacon, Fry, Fryable, Unfry, ciphers::Speck };
use rand::{ distributions::{ Alphanumeric }, Rng };

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    id: u32,
    name: String,
    bank_account: Bacon,
}
fn main() {
    // key from cli args
    let args: Vec<String> = std::env::args().collect();
    let mut key_str: String = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut rng = rand::thread_rng();
        rng.sample_iter(&Alphanumeric).take(16).collect()
    };
    drop(args);
    let key_128 = bacon::key_128(&key_str);
    key_str = "".to_string();               // emptying and
    drop(key_str);                          // dropping key

    // create partial fryable
    let fryable_bank_account = Fryable::from(vec!["First Moon Bank".to_string(), "IPBAN: M01A123456789".to_string()]);
    // create a person
    let fried_bank_account = Bacon::fry(fryable_bank_account, key_128);
    let p = Person{ id: 1234, name: "Dr Blofeld".to_string(), bank_account: fried_bank_account };

    dbg!(&p);
    
    let bank_account = Bacon::unfry::<Speck, Fryable>(p.bank_account, key_128).unwrap();
    dbg!(bank_account);
}