use _09_implement_pkcs7_padding::pad;
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
    let mut ciphertext = ciphertext.to_vec();
    ciphertext[43] ^= 1;
    ciphertext
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
        let plaintext = [[0u8; 16], *b"00000:admin<true"].concat();
        let ciphertext = oracle(&plaintext);
        let ciphertext = bitflip(&ciphertext);
        assert!(is_admin(&ciphertext));
    }
}
