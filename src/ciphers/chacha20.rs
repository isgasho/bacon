extern crate chacha;
use super::{ super::{ Bacon, BaconState }, Cipher, Decrypt, Encrypt };
use bigint::uint::U256;
use chacha::{ ChaCha, KeyStream };
pub struct ChaCha20{ key: [u8; 32], nonce: [u8; 8] }

impl Cipher for ChaCha20 {
    type Key = U256;
    type Cipher = Self;
    fn new(k: Self::Key) -> Self {
        let mut key = [0_u8; 32];
        k.to_little_endian(&mut key);
        let nonce = [1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8];
        ChaCha20 { key, nonce }
    }
}
impl Decrypt for ChaCha20 {
    fn decrypt(&self, bacon: Bacon) -> Bacon {
        self.encrypt(bacon)
    }
}
impl Encrypt for ChaCha20 {
    fn encrypt(&self, bacon: Bacon) -> Bacon {
        let mut chacha = ChaCha::new_chacha20(&self.key, &self.nonce);
        let mut data = vec![];
        for block in bacon.data.iter() {
            let mut buf = block.to_le_bytes();
            chacha.xor_read(&mut buf[..]).expect("hit end of stream far too soon");
            data.push(u128::from_le_bytes(buf));
        }
        Bacon { state: BaconState::Fried, descr: bacon.descr.clone(), data }
    }
}
