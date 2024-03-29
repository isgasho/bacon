//! The bacon crate is an interface to ease the implementation of new Ciphers.
//! Bacon is an adapter to the Speck and Chacha20 ciphers and provides functionality 
//! to en- and decrypt an arbitrary struct```struct T where T: Serialize + Deserialize```

#![forbid(unsafe_code)]
extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;

pub mod ciphers;

use serde::{ Deserialize, Serialize };
use std::{ collections::HashMap, fmt::{ Display, Formatter } };

/// Fried: Data stored in encrypted form. Unfried: The data is serialized but not encrypted.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BaconState {
    /// The data stored in Bacon is encrypted (and previously serialized)
    Fried,
    /// The data stored in Bacon is serialized not not encrypted
    Unfried
}
/// ```Bacon``` is a reusable wrapper for an arbitrarty serialized struct stored in the field ```data: Vec<u128>```
/// The optional description can be used to share information regarding the Bacon, that may be neccessary to
/// en-/decrypt a Bacon
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bacon { pub state: BaconState, pub descr: Option<HashMap<String,String>>, pub data: Vec<u128> }

impl Bacon {
    /// Create a new Bacon with State Fried | Unfried and d being the type that hold the data
    /// of the wrapped struct. Bacon serializes ```d: T``` into blocks in a Vec<u128>
    /// explicitly drops(d)
    pub fn new<T: for <'de> Deserialize<'de> + Serialize>(state: BaconState, descr: Option<HashMap<String,String>>, d: T) -> Bacon {
        let data = fry!(d);
        //drop(d); // chunks! drops d
        Bacon { state, descr, data }
    }
}

impl From<String> for Bacon {
    fn from(string:  String) -> Self {
        let data = fry!(string);
        Bacon { state: BaconState::Unfried, descr: None, data }
    }
}

impl From<Vec<String>> for Bacon {
    fn from(strings: Vec<String>) -> Self {
        let mut data: Vec<Vec<u128>> = vec![];
        for string in strings {
            data.push(fry!(string));
        }
        dbg!(data);
        Bacon { state: BaconState::Unfried, descr: None, data: vec![] }
     }
}

/// Utility function to turn a ```&str``` into a u128. The max length considered is 16 characters.
pub fn key_128(pass: &str) -> u128 {
    let mut x:  [u8; 16] = [0; 16];
    for (count, byte) in pass.as_bytes().iter().enumerate() {
        x[count] = *byte;
    }
    u128::from_le_bytes(x)
}

// TODO: should not be exported. implementing ciphers should use Bacon.data
#[macro_export]
macro_rules! fry {
    ($item:ident) => {
        {    // Using $item twice like this can cause an expression with side effects to be evaluated twice.
            let item = $item;
            let byte_doc = bincode::serialize(&item).unwrap();
            let chunks = byte_doc.chunks(16);
            let mut data: Vec<u128> = vec![];
            let mut x: [u8; 16] =  [0; 16];
            for chunk in chunks {
                let mut count = 0;
                for byte in chunk.clone() {
                    x[count] = *byte;
                    count += 1;
                }
                data.push( u128::from_le_bytes(x) );
            }
            data
        }
    }
}

#[macro_export]
macro_rules! unfry {
    ($bacon:ident, $target:ty) => {
        {
            let mut decr_bytes: Vec<u8> = vec![];
            for block in $bacon.data {
                for byte in u128::to_le_bytes(block).iter() {
                    decr_bytes.push(*byte);
                }    
            }  
            let decr: bincode::Result<$target> = bincode::deserialize(&decr_bytes);
            decr
        }
    }
}