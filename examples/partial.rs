#[forbid(unsafe_code)]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use bacon::{ Bacon, BaconState, ciphers::{ Cipher, speck::Speck, Encrypt } };
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

    let mut key_u128 = bacon::key_128(&key_str);
    key_str = "".to_string();               // emptying and
    drop(key_str);                          // dropping key str

    // create partial bacon. The data is an array representing a bank account
    // The bank account info is stored in serialized form, but not yet encrypted!
    // use bacon::{ Bacon, BaconState }
    let mut bcn_bank_account = Bacon::new(  
        BaconState::Unfried,
        None,
        ["First Moon Bank".to_string(), "IPBAN: M01A123456789".to_string()]
    );
    dbg!(&bcn_bank_account);
    // use ciphers::{ Cipher, speck::Speck, Decrypt, Encrypt }
    let cipher = Speck::new(key_u128);
    // Encrypt the bank account
    bcn_bank_account = cipher.encrypt(bcn_bank_account);
    // emptying and dropping key as soon as possible
    key_u128 = 0;
    drop(key_u128);
    // add the fried bacon to person
    let person = Person{ id: 1234, name: "Dr Blofeld".to_string(), bank_account: bcn_bank_account };
    dbg!(person);   
}