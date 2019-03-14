/*
#[cfg(test)]
mod tests {
    use super::{ super::{ Cipher, Decrypt, Encrypt }, Speck };
/*
    #[test]
    fn encrypt_decrypt() {
        for mut x in 0u128..90000 {
            // <3
            x = x.wrapping_mul(0x6eed0e9da4d94a4f6eed0e9da4d94a4f);
            x ^= (x >> 6) >> (x >> 122);
            x = x.wrapping_mul(0x6eed0e9da4d94a4f6eed0e9da4d94a4f);

            let speck: Speck = Speck::new(!x);

            assert_eq!(speck.decrypt(speck.encrypt(x)), x);
            //assert_eq!(speck.encrypt_block(x), encrypt_block(x, !x));
        }
    }
*/
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