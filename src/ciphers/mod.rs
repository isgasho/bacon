//! Module that contains various Ciphers to be used with Bacon.
#![forbid(unsafe_code)]
extern crate bigint;

use std::hash::{ Hash, Hasher };

pub mod speck;
pub mod chacha20;

use super::Bacon;
#[derive(Debug)]
pub struct MAC(u64);
/// trait Cipher must be implemented by Ciphers.
pub trait Cipher { 
    type Key;
    type Cipher;
    fn new(k: Self::Key, n: Option<[u8; 8]>) -> Self;
}
pub trait Authenticate : Hash {
    fn hash(&self, bacon: Bacon) -> MAC;
}
// TODO: return Result<Bacon, BaconError<T> 
pub trait Decrypt { fn decrypt(&self, bacon: Bacon) -> Bacon; }
pub trait Encrypt { fn encrypt(&self, bacon: Bacon) -> Bacon; }
