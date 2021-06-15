#[macro_use]
extern crate lazy_static;

use openssl::symm::{decrypt, encrypt, Cipher};
use std::sync::RwLock;

const BLOCK_SIZE: usize = 16;

lazy_static! {
    static ref KEY: RwLock<[u8; BLOCK_SIZE]> = RwLock::new([0u8; BLOCK_SIZE]);
    static ref IV: RwLock<[u8; BLOCK_SIZE]> = RwLock::new([0u8; BLOCK_SIZE]);
    static ref CIPHER: Cipher = Cipher::aes_128_cbc();
}

pub fn edit(ciphertext: &[u8], offset: usize, character: u8) -> Vec<u8> {
    let mut plaintext = decrypt(
        *CIPHER,
        &*KEY.read().unwrap(),
        Some(&*IV.read().unwrap()),
        ciphertext,
    )
    .unwrap();

    plaintext[offset] = character;

    encrypt(
        *CIPHER,
        &*KEY.read().unwrap(),
        Some(&*IV.read().unwrap()),
        &plaintext,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{edit, CIPHER, IV, KEY};
    use openssl::symm::encrypt;

    #[test]
    fn capture_the_flag() {
        {
            let mut w_key = KEY.write().unwrap();
            *w_key = rand::random();
            let mut w_iv = IV.write().unwrap();
            *w_iv = rand::random();
        }
        let plaintext = "absolutely arbitrary input";

        let ciphertext = encrypt(
            *CIPHER,
            &*KEY.read().unwrap(),
            Some(&*IV.read().unwrap()),
            plaintext.as_bytes(),
        )
        .unwrap();

        let mut result = "".to_string();
        for i in 0..plaintext.len() {
            for guess in 0u8..=255 {
                if edit(&ciphertext, i, guess) == ciphertext {
                    result.push(guess as char);
                }
            }
        }
        assert_eq!(result, plaintext);
    }
}
