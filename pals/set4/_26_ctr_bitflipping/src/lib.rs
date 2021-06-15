#[macro_use]
extern crate lazy_static;

use _09_implement_pkcs7_padding::pad;
use _19_break_fixed_nonce_ctr_mode_usong_substitutions::fixed_xor;
use openssl::symm::{decrypt, encrypt, Cipher};
use std::sync::RwLock;

const BLOCK_SIZE: usize = 16;

lazy_static! {
    static ref KEY: RwLock<[u8; BLOCK_SIZE]> = RwLock::new([0u8; BLOCK_SIZE]);
    static ref IV: RwLock<[u8; BLOCK_SIZE]> = RwLock::new([0u8; BLOCK_SIZE]);
    static ref CIPHER: Cipher = Cipher::aes_128_ctr();
}

fn oracle(plaintext: &[u8]) -> Vec<u8> {
    let prefix = b"comment1=cooking%20MCs;userdata=";
    let suffix = b";comment2=%20like%20a%20pound%20of%20bacon";

    let plaintext = String::from_utf8_lossy(plaintext).replace("\"", " ");
    let plaintext = plaintext.replace(";", " ");

    let plaintext = [prefix, plaintext.as_bytes(), suffix].concat();
    let plaintext = pad(&plaintext, BLOCK_SIZE as u8);

    encrypt(
        *CIPHER,
        &*KEY.read().unwrap(),
        Some(&*IV.read().unwrap()),
        &plaintext,
    )
    .unwrap()
}

fn bitflip(ciphertext: &[u8]) -> Vec<u8> {
    let c1 = ciphertext.get(0..BLOCK_SIZE).unwrap();
    let c2 = ciphertext.get(BLOCK_SIZE..).unwrap();

    let c1 = &fixed_xor(c1, &fixed_xor(b"comment1=cooking", b"___admin=true___"));

    [c1, c2].concat()
}

fn is_admin(ciphertext: &[u8]) -> bool {
    let plaintext = decrypt(
        *CIPHER,
        &*KEY.read().unwrap(),
        Some(&*IV.read().unwrap()),
        &ciphertext,
    )
    .unwrap();
    let plaintext = String::from_utf8_lossy(&plaintext);

    println!("{}", plaintext);

    if plaintext.contains("admin=true") {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{bitflip, is_admin, oracle, IV, KEY};

    #[test]
    fn capture_the_flag() {
        {
            let mut w_key = KEY.write().unwrap();
            *w_key = rand::random();
            let mut w_iv = IV.write().unwrap();
            *w_iv = rand::random();
        }
        let plaintext = b"absolutely arbitrary input that don\"t matter;";
        let ciphertext = oracle(plaintext);
        let ciphertext = bitflip(&ciphertext);
        assert!(is_admin(&ciphertext));
    }
}
