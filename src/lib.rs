#![forbid(unsafe_code)]
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use serde::{ Deserialize, Serialize };
pub mod speck;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bacon { pub data: Vec<u128> }
pub trait Fry { fn fry<T: Serialize>(source: T, key: u128) -> Bacon; }
pub trait Unfry { fn unfry<U: Cipher, T: for<'de> Deserialize<'de>>(bacon: Bacon, key: u128) -> bincode::Result<T>; }

// currently used to 
#[derive(Debug, Deserialize, Serialize)]
pub struct Fryable { data: Vec<String> }

impl From<Vec<String>> for Fryable {
    fn from(data:  Vec<String>) -> Self {
        Fryable { data }
    }
}
pub struct Speck;
pub trait Cipher {}
impl Cipher for Speck {}

impl Fry for Bacon {
    fn fry<T: Serialize>(source: T, key: u128) -> Bacon {
        fry!(source, key)
    }
}

impl Unfry for Bacon {
    fn unfry<U: Cipher, T: for<'de> Deserialize<'de>>(bacon: Bacon, key: u128) -> bincode::Result<T> {
        unfry!(bacon, T, key)
    }
}

/// returns a u128 from a 16 character str
pub fn key_128(pass: &str) -> u128 {
    let mut x:  [u8; 16] = [0; 16];
     for (count, byte) in pass.as_bytes().iter().enumerate() {
        x[count] = *byte;
    }
    u128::from_be_bytes(x)
}

#[macro_export]
macro_rules! fry {
    ($item:ident, $key:ident) => {
        {
            let key = speck::Key::new($key);
            let byte_doc = bincode::serialize(&$item).unwrap();
            let chunks = byte_doc.chunks(16);
            drop($item);
            let mut data: Vec<u128> = vec![];
            let mut x:  [u8; 16] =  [0; 16];
            for chunk in chunks {
                let mut count = 0;
                for byte in chunk.clone() {
                    x[count] = *byte;
                    count += 1;
                }
                data.push(key.encrypt_block( u128::from_be_bytes(x) ) );
            }
            Bacon { data: data }   
        }
    }
}

#[macro_export]
macro_rules! unfry {
    ($fried_bacon:ident, $target:ty, $key:ident) => {
        {
            let key = speck::Key::new($key);
            let mut decr_bytes: Vec<u8> = vec![];
            for chunk in $fried_bacon.data {
                for byte in u128::to_be_bytes(key.decrypt_block(chunk)).iter() {
                    decr_bytes.push(*byte);
                }    
            }  
            let decr: bincode::Result<$target> = bincode::deserialize(&decr_bytes);
            decr
        }
    }
}