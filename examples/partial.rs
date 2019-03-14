#[forbid(unsafe_code)]
#[macro_use] extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use bacon::{ Bacon, BaconState, ciphers::{ Cipher, speck::Speck, Decrypt, Encrypt } };
use rand::{ distributions::{ Alphanumeric }, Rng };
use std::collections::HashMap;
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
    let mut bank_account: HashMap<String, String> = HashMap::new();
    bank_account.insert("Account Holder".to_string(), "Dr Blofeld".to_string());
    bank_account.insert("Bank".to_string(), "First Moon Bank".to_string());
    bank_account.insert("IPBAN".to_string(), "IPBAN: M01A123456789".to_string());
    // Serialize the HashMap with Bacon
    let mut bcn_bank_account = Bacon::new(  
        BaconState::Unfried,
        None,
        bank_account
    );
    println!("Fried(serialized) bank_account:\n{:#?}", &bcn_bank_account);
    // use ciphers::{ Cipher, speck::Speck, Decrypt, Encrypt }
    let cipher = Speck::new(key_u128);
    // Encrypt the bank account
    bcn_bank_account = cipher.encrypt(bcn_bank_account);
    // emptying and dropping key as soon as possible
    key_u128 = 0;
    drop(key_u128);
    // add the fried bacon to person
    let person = Person{ id: 1234, name: "Dr Blofeld".to_string(), bank_account: bcn_bank_account };
    println!("A Person with partially encrypted data:\n{:#?}", person);
    // attempt to decrypt bank account info with wrong key
    let wrong_key_u128 =  bacon::key_128("dk-lf/.Mjl38Nhd!");
    let malicious = Speck::new(key_u128);
    let corrupted_bank_account = malicious.decrypt(person.bank_account.clone());
    // note that no error message is returned. The equal length of the blocks show that it has not been
    // successfully decrypted.
    println!("Decrypt attempt:\n{:#?}", corrupted_bank_account);
    // lets see attpemt to unfry the bacon into a HashMap fails
    match unfry!(corrupted_bank_account, HashMap<String, String>) {
        Ok(hash_map) => { dbg!(hash_map); },
        Err(e) => { dbg!(e); }
    }
    // Now an attempt with the key in the correct Speck object
    let ba =  cipher.decrypt(person.bank_account);
    // The blocks are the same as in the original Bacon
    dbg!(&ba);
    // unfrying is successful
    match unfry!(ba, HashMap<String, String>) {
        Ok(hash_map) => { dbg!(hash_map); },
        Err(e) => { dbg!(e); }
    }
}