//! The Speck implementation used in ```bacon``` is a fork from the [crate speck v1.1.0](https://docs.rs/crate/speck/1.1.0/source/src/lib.rs)
//! 
//! SPECK is a really simple block cipher designed by the NSA. It is famous for its simple
//! structure and code size, which can fit in just a couple of lines, while still preserving
//! security.
use super::{ super::{ Bacon, BaconState }, Cipher, Decrypt, Encrypt };

/// A single round of SPECK.
/// This is a keyed ARX transformation.
macro_rules! round {
    ($x:ident, $y:ident, $k:ident) => {
        $x = $x.rotate_right(8);
        $x = $x.wrapping_add($y);
        $x ^= $k as u64;
        $y = $y.rotate_left(3);
        $y ^= $x;
    }
}

/// Revert a SPECK round given some subkey.
macro_rules! inv_round {
    ($x:ident, $y:ident, $k:ident) => {
        $y ^= $x;
        $y = $y.rotate_right(3);
        $x ^= $k;
        $x = $x.wrapping_sub($y);
        $x = $x.rotate_left(8);
    }
}

/// The Speck Cipher
pub struct Speck { schedule: [u64; 32 as usize], }

impl Cipher for Speck {
    type Key = u128;
    type Cipher = Speck;
    fn new(k: Self::Key) -> Self {
        let mut k1 = (k >> 64) as u64;
        let mut k2 = k as u64;

        let mut ret = Speck { schedule: [0; 32 as usize], };
        // Run `ROUNDS - 1` rounds to generate the key's endpoint (the last key in the schedule).
        println!("{:?}", ret.schedule.len());
    
        for i in 0..ret.schedule.len() {
            // Insert the key into the schedule.
            ret.schedule[i as usize] = k2;
            // The beautiful thing about SPECK is that it reuses its round function to generate the
            // key schedule.
            round!(k1, k2, i);
        }
        ret       
    }
}

impl Decrypt for Speck {
    /// Decrypt a 128-bit block with this key.
    fn decrypt(&self, bacon: Bacon) -> Bacon {
        let mut data = vec![];
        for block in &bacon.data {
            let mut c1 = (block >> 64) as u64;
            let mut c2 = (block >> 64) as u64;
            // We run a round for every subkey in the generated key schedule.
            for &k in self.schedule.iter().rev() {
                // Run a round on the message.
                inv_round!(c1, c2, k);
            }
            data.push(u128::from(c2) | u128::from(c1) << 64);
        }
        Bacon { state: BaconState::Unfried, descr: bacon.descr.clone(), data }
    }
}

impl Encrypt for Speck {
    /// Encrypt a 128-bit block with this key.
    fn encrypt(&self, bacon: Bacon) -> Bacon {
        let mut data = vec![];
        for block in &bacon.data {
            let mut m1 = (block >> 64) as u64;
            let mut m2 = (block >> 64) as u64;
            // We run a round for every subkey in the generated key schedule.
            for &k in &self.schedule {
                // Run a round on the message.
                round!(m1, m2, k);
            }
            data.push(u128::from(m2) | u128::from(m1) << 64);
        }
        Bacon { state: BaconState::Fried, descr: bacon.descr.clone(), data }
    }
}