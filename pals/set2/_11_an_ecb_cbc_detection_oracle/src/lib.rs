#![feature(array_methods)]

use _09_implement_pkcs7_padding::pad;
use openssl::symm::{encrypt, Cipher};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn ecb_score(ciphertext: &[u8], chunk_size: usize) -> u32 {
    let mut map = HashMap::new();

    for block in ciphertext.chunks(chunk_size) {
        match map.get(block) {
            None => {
                map.insert(block, 0);
            }
            Some(value) => {
                let new_val = value + 1;
                map.insert(block, new_val);
            }
        }
    }
    map.values().sum::<u32>()
}

fn encryption_oracle(plaintext: &[u8]) -> (Vec<u8>, &str) {
    let mut rng = thread_rng();

    let prefix: Vec<u8> = vec![0; rng.gen_range(5..11)]
        .into_iter()
        .map(|_| rand::random())
        .collect();

    let suffix: Vec<u8> = vec![0; rng.gen_range(5..11)]
        .into_iter()
        .map(|_| rand::random())
        .collect();
    let plaintext = [&prefix, plaintext, &suffix].concat();

    let plaintext = pad(&plaintext, 16);

    let key: [u8; 16] = rand::random();

    let iv: [u8; 16] = rand::random();

    let (cipher, iv, mode) = match rand::random() {
        true => {
            println!("ECB");
            (Cipher::aes_128_ecb(), None, "ECB")
        }
        false => {
            println!("CBC");
            (Cipher::aes_128_cbc(), Some(iv.as_slice()), "CBC")
        }
    };

    (encrypt(cipher, &key, iv, &plaintext).unwrap(), mode)
}

fn detect_ecb(ciphertext: &[u8]) -> &str {
    if ecb_score(ciphertext, 16) > 0 {
        return "ECB";
    }
    "CBC"
}

#[cfg(test)]
mod tests {
    use crate::{detect_ecb, encryption_oracle};

    #[test]
    fn test_detect_ecb() {
        let plaintext = b"YELLOW SUBMARINE".repeat(3);
        let (ciphertext, mode) = encryption_oracle(plaintext.as_slice());
        assert_eq!(detect_ecb(&ciphertext), mode);
    }
}
