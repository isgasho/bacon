#[forbid(unsafe_code)]
extern crate bacon;

use bacon::{ Bacon, Fry, Fryable, Unfry, ciphers::Speck }; // a generic wrapper 
/// $ cargo run --example command_line {mandatory secret_key} ["messages"]
/// Example: cargo run --example command_line jh7dhezsgh56,.kL "This is a secret message" "This is some payload"
fn main() {
    // key from cli args
    let  mut args: Vec<String> = std::env::args().collect();
    let key_128 = bacon::key_128(&args[1]);
    args.drain(0..2);  // that is the program name and secret
    let fryable = Fryable::from(args);
    dbg!(&fryable);
    let bacon = Bacon::fry(fryable, key_128);
    dbg!(&bacon);
    let f = Bacon::unfry::<Speck, Fryable>(bacon, key_128).unwrap();
    dbg!(f);
}