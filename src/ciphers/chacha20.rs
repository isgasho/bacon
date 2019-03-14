//! Bacon does not provide an own imiplementation. The ChaCha20 Cipher used is the
//! [crate chacha v0.3.0](https://docs.rs/chacha/0.3.0/chacha/)
extern crate chacha;
extern crate rand;
use super::{ super::{ Bacon, BaconState }, Authenticate, Cipher, Decrypt, Encrypt, MAC };
use bigint::uint::U256;
use chacha::{ ChaCha, KeyStream };
use rand::Rng;
use std::{ collections::hash_map::DefaultHasher, hash::{ Hash, Hasher }, ops::{ BitOr, BitXor} };

#[derive(Hash)]
pub struct ChaCha20{ key: [u8; 32], nonce: [u8; 8] }
impl Cipher for ChaCha20 {
    type Key = U256;
    type Cipher = Self;
    fn new(k: Self::Key, n: Option<[u8; 8]>) -> Self {
        let mut key = [0_u8; 32];
        k.to_little_endian(&mut key);
        let mut nonce: [u8; 8] = [1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8];
        if n.is_some() { nonce = n.unwrap(); }
        for i in 0..8 {
           nonce[i] = rand::thread_rng().gen_range(0, 255);
        }
        // dbg!( &nonce );
        ChaCha20 { key, nonce }
    }
}
struct BlockSize(u8);
impl Authenticate for ChaCha20 {  // TODO: Hash not impl for HashMap<String, String> of Bacon
    fn hash(&self, bacon: Bacon)-> MAC {
        let mut hasher = DefaultHasher::new();
        self.key.hash(&mut hasher);
        self.nonce.hash(&mut hasher);
        let kh = hasher.finish();
        let mut o_k: u64 = kh.bitxor(0x5c * 64);
        let mut i_k: u64 = kh.bitxor(0x36 * 64);
        let mut b_k: u64 = 0;
        for block in &bacon.data {
            block.hash(&mut hasher);
            b_k ^= b_k | hasher.finish();              
        }
        bacon.data.hash(&mut hasher);

        ( o_k | (i_k | hasher.finish() ) ).hash(&mut hasher);
  
        MAC( hasher.finish() )
    }
}

impl Decrypt for ChaCha20 {
    fn decrypt(&self, bacon: Bacon) -> Bacon {
        self.encrypt(bacon) // TODO: state is wrong should be unfried from here
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
