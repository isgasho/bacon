//! Module that contains various Ciphers to be used with Bacon.
#![forbid(unsafe_code)]
extern crate bigint;

use std::{ fmt::Display, hash::{ Hash, Hasher } };

pub mod speck;
pub mod chacha20;

use super::Bacon;

/// A default a Cipher is used for hashing, rather for encryption and security is not the main concern.
/// ie. to pad a message 
const DEFAULT_NONCE: [u8; 8] = [1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8];

#[derive(Debug)]
pub struct MAC(u64);
/// trait Cipher must be implemented by Ciphers.
pub trait Cipher { 
    type Key;
    type Cipher;
    fn new(k: Self::Key, n: Nonce) -> Self;
}
pub trait Authenticate : Hash {
    fn hash(&self, bacon: &Bacon) -> MAC;
}
// TODO: return Result<Bacon, BaconError<T> 
pub trait Decrypt { fn decrypt(&self, bacon: Bacon) -> Bacon; }
pub trait Encrypt { fn encrypt(&self, bacon: Bacon) -> Bacon; }

impl Display for MAC {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
/// BaconDefault can be used where ChaCha20 is used as Hasher
#[derive(Hash)]
pub enum Nonce {
    BaconDefault,
    Custom([u8; 8]),
    None,
    Rand
}