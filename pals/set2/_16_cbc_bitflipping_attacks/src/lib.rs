use _09_implement_pkcs7_padding::pad;
use _19_break_fixed_nonce_ctr_mode_usong_substitutions::fixed_xor;
use openssl::symm::{decrypt, encrypt, Cipher};

static mut KEY: [u8; 16] = [0u8; 16];

fn oracle(plaintext: &[u8]) -> Vec<u8> {
    let prefix = b"comment1=cooking%20MCs;userdata=";
    let suffix = b";comment2=%20like%20a%20pound%20of%20bacon";

    let plaintext = [prefix, plaintext, suffix].concat();
    let plaintext = pad(&plaintext, 16);

    let cipher = Cipher::aes_128_cbc();

    encrypt(cipher, &unsafe { KEY }, Some(&[0u8; 16]), &plaintext).unwrap()
}

fn bitflip(ciphertext: &[u8]) -> Vec<u8> {
    let c1 = ciphertext.get(0..16).unwrap();
    let c2 = ciphertext.get(16..32).unwrap();
    let c3 = ciphertext.get(32..).unwrap();

    let c1 = &fixed_xor(c1, &fixed_xor(b"%20MCs;userdata=", b"___admin=true___"));

    [c1, c2, c3].concat()
}

fn is_admin(ciphertext: &[u8]) -> bool {
    let cipher = Cipher::aes_128_cbc();

    let plaintext = decrypt(cipher, &unsafe { KEY }, Some(&[0u8; 16]), &ciphertext).unwrap();
    let plaintext = String::from_utf8_lossy(&plaintext);

    if plaintext.contains("admin=true") {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{bitflip, is_admin, oracle, KEY};

    #[test]
    fn capture_the_flag() {
        unsafe {
            KEY = rand::random();
        }
        let plaintext = b"absolutely arbitrary input that dont matter";
        let ciphertext = oracle(plaintext);
        let ciphertext = bitflip(&ciphertext);
        assert!(is_admin(&ciphertext));
    }
}
