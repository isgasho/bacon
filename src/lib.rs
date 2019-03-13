//! The bacon crate provides functionality to en- and decrypt (called frying and unfrying),
//! arbitrary ```struct T where T: Serialize + Deserialize```

#[forbid(unsafe_code)]
extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;
use serde::{ Deserialize, Serialize };
use ciphers::speck::Speck;

pub mod ciphers;

/// ```Bacon``` a wrapper for an encrypted struct (called bacon) stored in the field ```data: Vec<u128>```
/// Implements ```Fry``` and ```Unfry```. Cannot fry or unfry itself. (may change in the future).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bacon { pub data: Vec<u128> }
/// Marker trait for Ciphers supported by Bacon
pub trait Cipher {} // to be implemented by Speck, ChaCha etc
/// Provides methods to fry a struct. Return fried ```Bacon```
pub trait Fry { fn fry<T: Serialize>(source: T, key: u128) -> Bacon; }
/// Provides methods to unfry a fried Bacon. Return bincode::Result<T>
pub trait Unfry { fn unfry<U: Cipher, T: for<'de> Deserialize<'de>>(bacon: Bacon, key: u128) -> bincode::Result<T>; }

/// A wrapper to support Fyring of Strings. Can also be used to enrcypt messages from the command line.
/// See example ```command_line.rs```
#[derive(Debug, Deserialize, Serialize)]
pub struct Fryable { data: Vec<String> }

/// Preferred way of creating a Fryable
impl From<Vec<String>> for Fryable { fn from(data:  Vec<String>) -> Self { Fryable { data } } }
impl Fry for Bacon { fn fry<T: Serialize>(source: T, key: u128) -> Bacon { fry!(source, key) } }
impl Unfry for Bacon {
    fn unfry<U: Cipher, T: for<'de> Deserialize<'de>>(bacon: Bacon, key: u128) -> bincode::Result<T> {
        unfry!(bacon, T, key)
    }
}

/// Utility function to turn a ```&str``` into a u128. The max length considered is 16 characters.
pub fn key_128(pass: &str) -> u128 {
    let mut x:  [u8; 16] = [0; 16];
     for (count, byte) in pass.as_bytes().iter().enumerate() {
        x[count] = *byte;
    }
    u128::from_be_bytes(x)
}
/// Fry an arbitrary T: Serialize. Does only support ```ciphers::speck::Speck```
#[macro_export]
macro_rules! fry {
    ($item:ident, $key:ident) => {
        {
            let key = Speck::new($key);
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
            drop($key);
            Bacon { data: data }   
        }
    }
}

/// Unfry an arbitrary T: Serialize. Does only support ```ciphers::speck::Speck``` 
#[macro_export]
macro_rules! unfry {
    ($fried_bacon:ident, $target:ty, $key:ident) => {
        {
            let key = Speck::new($key);
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