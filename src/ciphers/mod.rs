//! Module that contains various Ciphers to be used with Bacon.
#![forbid(unsafe_code)]
extern crate bigint;

pub mod speck;
pub mod chacha20;

use super::Bacon;
/// trait Cipher must be implemented by Ciphers.
pub trait Cipher { 
    type Key;
    type Cipher;
    fn new(k: Self::Key) -> Self;
}

pub trait Decrypt { fn decrypt(&self, bacon: Bacon) -> Bacon; }
pub trait Encrypt { fn encrypt(&self, bacon: Bacon) -> Bacon; }