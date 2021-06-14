#[macro_use]
extern crate lazy_static;

const BLOCK_SIZE: usize = 16;

use std::sync::RwLock;

lazy_static! {
    static ref KEY: RwLock<[u8; BLOCK_SIZE]> = RwLock::new([0u8; BLOCK_SIZE]);
    static ref IV: RwLock<[u8; BLOCK_SIZE]> = RwLock::new([0u8; BLOCK_SIZE]);
}

#[cfg(test)]
mod tests {
    use crate::{IV, KEY};
    use _19_break_fixed_nonce_ctr_mode_usong_substitutions::fixed_xor;
    use openssl::symm::{decrypt, encrypt, Cipher};

    #[test]
    fn capture_the_flag() {
        {
            let mut w_key = KEY.write().unwrap();
            *w_key = rand::random();
        }
        {
            let mut w_iv = IV.write().unwrap();
            *w_iv = *KEY.read().unwrap();
        }
        let plaintext = b"absolutely secret arbitrary data that don\'t matter;abcdasdfasdfsdf";

        println!("{}", String::from_utf8_lossy(plaintext));

        let cipher = Cipher::aes_128_cbc();

        let ciphertext = encrypt(
            cipher,
            &*KEY.read().unwrap(),
            Some(&*IV.read().unwrap()),
            plaintext,
        )
        .unwrap();

        let derived_ciphertext = [
            ciphertext.get(0..16).unwrap(),
            &[0u8; 16],
            ciphertext.get(0..16).unwrap(),
            ciphertext.get(48..).unwrap(),
        ]
        .concat();

        let derived_plaintext = decrypt(
            cipher,
            &*KEY.read().unwrap(),
            Some(&*IV.read().unwrap()),
            &derived_ciphertext,
        )
        .unwrap();

        let iv_key = fixed_xor(
            derived_plaintext.get(0..16).unwrap(),
            derived_plaintext.get(32..48).unwrap(),
        );

        let my_plaintext = decrypt(cipher, &*iv_key, Some(&*iv_key), &ciphertext).unwrap();
        println!("{}", String::from_utf8_lossy(&my_plaintext));

        assert_eq!(plaintext.to_vec(), my_plaintext);
    }
}
