//! The Speck implementation used in ```bacon``` is a fork from the [crate speck v1.1.0](https://docs.rs/crate/speck/1.1.0/source/src/lib.rs)
//! 
//! SPECK is a really simple block cipher designed by the NSA. It is famous for its simple
//! structure and code size, which can fit in just a couple of lines, while still preserving
//! security.
use super::{ Cipher, Decrypt, Encrypt };

/// A single round of SPECK.
/// This is a keyed ARX transformation.
macro_rules! round {
    ($x:ident, $y:ident, $k:ident) => {
        $x = $x.rotate_right(8);
        $x = $x.wrapping_add($y);
        $x ^= $k;
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

const ROUNDS: u64 = 32;
/// The Speck Cipher
pub struct Speck { schedule: [u64; ROUNDS as usize], }

impl Cipher for Speck {
    type Key = u128;
    type Cipher = Speck;
    fn new(k: Self::Key) -> Self {
        let mut k1 = (k >> 64) as u64;
        let mut k2 = k as u64;

        let mut ret = Speck { schedule: [0; 32 as usize], };
        // Run `ROUNDS - 1` rounds to generate the key's endpoint (the last key in the schedule).
        println!("{:?}", ret.schedule.len());
    
        for i in 0..ROUNDS {
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
    fn decrypt_block(&self, c: u128) -> u128 {
        let mut c1 = (c >> 64) as u64;
        let mut c2 = c as u64;
        // We run a round for every subkey in the generated key schedule.
        for &k in self.schedule.iter().rev() {
            // Run a round on the message.
            inv_round!(c1, c2, k);
        }
        u128::from(c2) | u128::from(c1) << 64
    }
}

impl Encrypt for Speck {
    /// Encrypt a 128-bit block with this key.
    fn encrypt_block(&self, m: u128) -> u128 {
        let mut m1 = (m >> 64) as u64;
        let mut m2 = m as u64;
        // We run a round for every subkey in the generated key schedule.
        for &k in &self.schedule {
            // Run a round on the message.
            round!(m1, m2, k);
        }
        u128::from(m2) | u128::from(m1) << 64
    }
}


#[cfg(test)]
mod tests {
    use super::{ super::{ Cipher, Decrypt, Encrypt }, Speck };

    #[test]
    fn encrypt_decrypt() {
        for mut x in 0u128..90000 {
            // <3
            x = x.wrapping_mul(0x6eed0e9da4d94a4f6eed0e9da4d94a4f);
            x ^= (x >> 6) >> (x >> 122);
            x = x.wrapping_mul(0x6eed0e9da4d94a4f6eed0e9da4d94a4f);

            let speck: Speck = Speck::new(!x);

            assert_eq!(speck.decrypt_block(speck.encrypt_block(x)), x);
            //assert_eq!(speck.encrypt_block(x), encrypt_block(x, !x));
        }
    }

    // #[test]
    // fn test_vectors() {
    //     // These test vectors are taken from the SPECK paper.
    //     assert_eq!(
    //         encrypt_block(
    //             0x6c617669757165207469206564616d20,
    //             0x0f0e0d0c0b0a09080706050403020100
    //         ),
    //         0xa65d9851797832657860fedf5c570d18
    //     );
    // }
}


/*
/// Encrypt a block with key schedule generated on-the-go.
///
/// This works great for one-time use of a key (such as usages other than encryption), because it
/// should never read from memory (both the message and the keys are stored in the registers). As
/// such, this should be really fast for such usage.
///
/// If you want to reuse the key, however, it is recommended that you use the precomputed schedule
/// provided by the `Key` struct.
pub fn encrypt_block(m: u128, k: u128) -> u128 {
    let mut m1 = (m >> 64) as u64;
    let mut m2 = m as u64;
    let mut k1 = (k >> 64) as u64;
    let mut k2 = k as u64;

    // Run the initial round (similar to the loop below, but doesn't update the key schedule).
    round!(m1, m2, k2);

    for i in 0..ROUNDS - 1 {
        // Progress the key schedule.
        round!(k1, k2, i);
        // Run a round over the message.
        round!(m1, m2, k2);
    }

    u128::from(m2) | u128::from(m1) << 64
}
*/