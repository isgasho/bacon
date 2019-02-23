#[macro_use]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use bacon::Bacon;
use rand::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct SuperSecretDocument<'a> {
    content: &'a str
}

impl <'a>SuperSecretDocument<'a> {
    fn new(content: &str) -> SuperSecretDocument {
        SuperSecretDocument { content }
    }
}

// encrypts a struct using the speck algorithm and decrypts it back
fn main() {
    let ssd = SuperSecretDocument::new("Hello World. How are you?");
    println!("{:#?}", &ssd);
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(u128::min_value(), u128::max_value());
    let encr_chunks: Bacon = fry!(ssd, key);
    println!("encrypted chunks: {:#?}", encr_chunks);
    unfry!(encr_chunks, SuperSecretDocument, key);
}
