#[macro_use] extern crate bacon;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bincode;
use bacon::{ Bacon, BaconState, ciphers::{ Cipher, speck::Speck, Decrypt, Encrypt } };

#[derive(Serialize)]
enum Sex { HotFemale, HotterFemale }
#[derive(Serialize)]
struct Dancer {
    name: String,
    favorite_dance: String,
    age: u8,
    sex: Sex
}
fn main() {
    let dancer = Dancer {
        name: "SriChaCa Dunzapawn".to_string(),
        favorite_dance: "Two-Step".to_string(),
        age: 18,
        sex: Sex::HotterFemale
    };
    let k: u128 = u128::max_value();
    let cipher: Speck = Speck::new(k);
    let mut bacon = Bacon::new(BaconState::Unfried, dancer);
    dbg!(&bacon);
    bacon = cipher.encrypt(bacon);
    dbg!(&bacon);
    bacon = cipher.decrypt(bacon);
    dbg!(&bacon);
}