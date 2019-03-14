#[macro_use] extern crate bacon;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bincode;
use bacon::{ Bacon, BaconState, ciphers::{ Cipher, chacha20::ChaCha20, Decrypt, Encrypt } };

#[derive(Deserialize, Serialize)]
enum Sex { HotFemale, HotterFemale }
#[derive(Deserialize, Serialize)]
struct Dancer {
    name: String,
    favorite_dance: String,
    age: u8,
    sex: Sex
}
fn main() {
    let dancer = Dancer {
        name: "SriChaCha Dunzapawn".to_string(),
        favorite_dance: "Two-Step".to_string(),
        age: 18,
        sex: Sex::HotterFemale
    };
    let k = bigint::uint::U256::MAX;
    let cipher: ChaCha20 = ChaCha20::new(k);
    let mut bacon = Bacon::new(BaconState::Unfried, None, dancer);
    dbg!(&bacon);
    bacon = cipher.encrypt(bacon);
    dbg!(&bacon);
    bacon = cipher.decrypt(bacon);
    dbg!(&bacon);
}
