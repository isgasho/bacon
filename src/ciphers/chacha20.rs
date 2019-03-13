use super::{ Cipher, Decrypt, Encrypt };
use bigint::uint::U256;
pub struct ChaCha20{ key: [u8; 32], nonce: [u8; 8] }

impl Cipher for ChaCha20 {
    type Key = U256;
    type Cipher = Self;
    fn new(k: Self::Key) -> Self {
        let mut keys = [0_u8; 32];
        k.to_little_endian(&mut keys);
        let nonce = [0u8; 8];
        ChaCha20 {
            key: keys,
            nonce: nonce
        }
    }
}
impl Decrypt for ChaCha20 {
    fn decrypt_block(&self, c: u128) -> u128 {
        123_u128
    }
}
impl Encrypt for ChaCha20 {
    fn encrypt_block(&self, m: u128) -> u128 { 234_u128 }
}
