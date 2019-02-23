extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate speck;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bacon {
    pub data: Vec<Vec<u128>>
}

/// serializes and encrypts an item given item and a u128 key
/// returns fried items
#[macro_export]
macro_rules! fry {
    ($item:ident, $key:ident) => {
        {
            let key = speck::Key::new($key);
            let byte_doc = bincode::serialize(&$item).unwrap();
            let chunks = byte_doc.chunks(16);
            drop($item);
            let mut encr_chunks: Vec<Vec<u128>> = vec![];
            for (i, chunk) in chunks.enumerate() {
                //dbg!(chunk);
                //println!("{:?}: {:?}", i, chunk);
                encr_chunks.push(vec![]);
                for byte in chunk {
                    encr_chunks[i].push( key.encrypt_block(*byte as u128) );
                }
                drop(chunk);
            }
            Bacon { data: encr_chunks }   
        }
    }
}

/// decrypts a with fry! encrypted item and deserializes into given type
#[macro_export]
macro_rules! unfry {
    ($fried_bacon:ident, $struct:ty, $key:ident) => {
        {
            let key = speck::Key::new($key);
            let mut decr_chunks: Vec<u8> = vec![];
            for chunk in $fried_bacon.data {
                for encr_byte in chunk {
                    decr_chunks.push( key.decrypt_block(encr_byte) as u8 );
                }
            } 
            let deserialized: $struct = bincode::deserialize(&decr_chunks).unwrap();
            println!("{:#?}", deserialized);
            //deserialized
        }
    }
}