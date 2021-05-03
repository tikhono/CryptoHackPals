use ascii::IntoAsciiString;
use openssl::symm::{encrypt, Cipher};
use std::io;
use std::io::Write;

static mut KEY: [u8; 16] = [0u8; 16];

pub fn get_cipher_text(plaintext: String) -> String {
    let cipher = Cipher::aes_128_ecb();

    let mut plaintext = hex::decode(plaintext).unwrap();
    plaintext.append(&mut "YELLOWSUBMARINE".as_bytes().to_vec());

    hex::encode(
        encrypt(cipher, unsafe { &KEY }, None, &plaintext)
            .unwrap()
            .as_slice(),
    )
}

pub fn recover_byte(
    target: String,
    guess: String,
    block_number: usize,
    block_size: usize,
) -> Option<String> {
    for i in 0x20..=0x7E {
        let byte = hex::encode([i]);
        let current_guess = get_cipher_text(guess.clone() + &byte);
        if target[..block_size * 2 * (block_number + 1)]
            == current_guess[..block_size * 2 * (block_number + 1)]
        {
            print!("{}", i as char);
            io::stdout().flush().unwrap();
            return Some(byte);
        }
    }
    None
}

pub fn receive_blocks(blocks_number: usize, block_size: usize) -> String {
    let mut append = "00".to_string().repeat(block_size * (blocks_number + 1));
    let mut plaintext = "".to_string();

    for _i in 0..block_size * (blocks_number + 1) - 1 {
        append = append.get(2..).unwrap().to_string();
        let byte = recover_byte(
            get_cipher_text(append.clone()),
            append.clone() + &plaintext,
            blocks_number,
            block_size,
        );
        match byte {
            None => {
                println!("Failed to find next byte");
                break;
            }
            Some(char) => {
                plaintext.push_str(&char);
            }
        }
    }
    hex::decode(plaintext.clone())
        .unwrap()
        .into_ascii_string()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{receive_blocks, KEY};

    #[test]
    fn capture_the_flag() {
        unsafe {
            KEY = rand::random();
        }
        assert_eq!(receive_blocks(2, 16), "YELLOWSUBMARINE");
    }
}
