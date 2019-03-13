//! Module that contains various Ciphers to be used with Bacon.
#![forbid(unsafe_code)]
extern crate bigint;

pub mod speck;
pub mod chacha20;

/// trait Cipher must be implemented by Ciphers.
pub trait Cipher { 
    type Key;
    type Cipher;
    fn new(k: Self::Key) -> Self;
}

pub trait Decrypt { fn decrypt(&self, c: u128) -> u128; }
pub trait Encrypt { fn encrypt(&self, m: u128) -> u128; }