#[forbid(unsafe_code)]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use bacon::{ Bacon, Fry, Fryable, Speck, Unfry }; // a generic wrapper 
use rand::{ distributions::{ Alphanumeric }, Rng };

/// $ cargo run --example command_line {secret_key} "[messages]"
/// Example: cargo run --example command_line jh7dhezsgh56,.kL "This is a secret message" "This is some payload"
fn main() {
    // key from cli args
    let args: Vec<String> = std::env::args().collect();
    let mut key_str: String = args[1].clone();
    let key_128 = bacon::key_128(&key_str);
    let fryable = Fryable::from(args);

    let bacon = Bacon::fry(fryable, key_128);
    key_str = "".to_string();               // emptying and
    drop(key_str);                          // dropping key
    dbg!(&bacon);
    let f = Bacon::unfry::<Speck, Fryable<T>>(bacon, key_128).unwrap();
}