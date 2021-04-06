use ascii::IntoAsciiString;
use std::io;
use std::io::Write;

pub fn get_cipher_text(plaintext: String) -> String {
    use curl::easy::Easy;
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let mut addr = "http://aes.cryptohack.org/ecb_oracle/encrypt/".to_string();
    addr.push_str(&*plaintext);
    addr.push_str("/");
    handle.url(&*addr).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    let ciphertext = std::str::from_utf8(&*data)
        .unwrap()
        .strip_prefix("{\"ciphertext\":\"")
        .unwrap()
        .strip_suffix("\"}\n")
        .unwrap()
        .to_string();
    ciphertext
}

pub fn recover_byte(
    target: String,
    guess: String,
    block_number: usize,
    block_size: usize,
) -> Option<String> {
    for i in 0x20..=0x7E {
        let byte = hex::encode([i]);
        let current_guess = get_cipher_text(guess.clone() + &*byte);
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
            append.clone() + &*plaintext,
            blocks_number,
            block_size,
        );
        match byte {
            None => {
                println!("Failed to find next byte");
                break;
            }
            Some(char) => {
                plaintext.push_str(&*char);
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
    use crate::receive_blocks;

    #[test]
    fn capture_the_flag() {
        let mut plaintext = "".to_string();
        plaintext.push_str(&*receive_blocks(2, 16));
        println!("{:?}", plaintext);
    }
}
