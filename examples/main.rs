#[macro_use]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use bacon::Bacon;
use rand::prelude::*;

// currently not working due to borrowed content

// #[derive(Clone, Debug, Deserialize, Serialize)]
// struct SuperSecretDocument<'a> {
//     content: &'a str
// }

// impl <'a>SuperSecretDocument<'a> {
//     fn new(content: &str) -> SuperSecretDocument {
//         SuperSecretDocument { content }
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    gender: char
}

// encrypts a struct using the speck algorithm and decrypts it back
fn main() {
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(u128::min_value(), u128::max_value());

    // let ssd = SuperSecretDocument::new("Hello World. How are you?");
    // println!("{:#?}", &ssd);

    // let fried_bacon: Bacon = fry!(ssd, key);
    // println!("{:#?}", &fried_bacon);
    
    // unfry!(fried_bacon, SuperSecretDocument, key);

    let my = Person { name: "Alice".to_string(), age: 7, gender: 'f' };
    dbg!(&my);

    let fried_bacon: Bacon = fry!(my, key);
    dbg!(&fried_bacon);
    let p = unfry!(fried_bacon, Person, key);
    dbg!(p);
}
