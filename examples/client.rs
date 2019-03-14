#[macro_use] extern crate bacon;
extern crate bincode;

use bacon::{ Bacon, BaconState, ciphers::{ Cipher, speck::Speck, Encrypt } };
use std::{ collections::HashMap, io::prelude::*, net::TcpStream };

// $ cargo run --example client 127.0.0.1:64100 17ZhjI3j/dshn3Kj "Super secreat message"
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    //dbg!(&args[2]);
    // second argument to be the secret key known to user and server
    let mut key_u128 = bacon::key_128(&args[2]); 
    //dbg!(key_u128);
    let cipher: Speck = Speck::new(key_u128, None);
    key_u128 = 0;
    //drop(key_u128);
    // third argument to be the secret message
    let mut descr: HashMap<String, String> = HashMap::new();
    descr.insert("Cipher".to_string(), "bacon::ciphers::speck::Speck".to_string());
    descr.insert("Type".to_string(), "std::string::String".to_string());
    let mut bacon = Bacon::new(BaconState::Unfried, Some(descr), args[3].clone());
    //dbg!(&bacon);
    bacon = cipher.encrypt(bacon);
    //dbg!(&bacon);
    // send to server
    let ser: Vec<u8> = bincode::serialize(&bacon).unwrap();
    //dbg!(&ser);
    //drop(bacon);
    let mut stream = TcpStream::connect("127.0.0.1:64100")?;
    stream.write(&ser)?;
    stream.read(&mut [0; 512])?;
    Ok(())
}