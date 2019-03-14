#![forbid(unsafe_code)]
extern crate bacon;

use bacon::{ Bacon, ciphers::speck::Speck }; // a generic wrapper 
/// $ cargo run --example command_line {mandatory secret_key} ["messages"]
/// Example: cargo run --example command_line jh7dhezsgh56,.kL "This is a secret message" "This is some payload"
fn main() {
    // key from cli args
    let  mut args: Vec<String> = std::env::args().collect();
    let mut key_128 = bacon::key_128(&args[1]);
    args.drain(0..2);  // that is the program name and secret

    let bacon = Bacon::from("Hello this is a String".to_string());
    dbg!(bacon);
    /*
    let bacon = Bacon::fry::<Speck, _>(fryable, key_128);
    dbg!(&bacon);
    let f = Bacon::unfry::<Speck, Fryable>(bacon, key_128).unwrap();
    // always "empty" sensitive data and drop early
    key_128 = 0_u128;
    drop(key_128);
    dbg!(f);
    */
}