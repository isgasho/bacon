#[forbid(unsafe_code)]
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate speck;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bacon { pub data: Vec<u128> }

/// returns a u128 from a 16 character str
pub fn key_128(pass: &str) -> u128 {
    let mut x:  [u8; 16] = [0; 16];
    let mut count = 0;
    for byte in pass.as_bytes() {
        x[count] = *byte;
        count += 1;
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
            for (i, chunk) in chunks.enumerate() {
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

/// decrypts a with fry! encrypted item and deserializes into given type
#[macro_export]
macro_rules! unfry {
    ($fried_bacon:ident, $struct:ty, $key:ident) => {
        {
            let key = speck::Key::new($key);
            let mut decr_bytes: Vec<u8> = vec![];
            for chunk in $fried_bacon.data {
                for byte in u128::to_be_bytes(key.decrypt_block(chunk)).iter() {
                    decr_bytes.push(*byte);
                }       
            } 
            let decr: bincode::Result<$struct> = bincode::deserialize(&decr_bytes);
            decr
        }
    }
}