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

pub fn recover_byte(target: String, guess: String) -> Option<String> {
    for i in 0..255 {
        let byte = hex::encode([i]);
        let current_guess = get_cipher_text(guess.clone() + &*byte);
        if target[0..31] == current_guess[0..31] {
            /*
            println!("{}", byte);
            println!(
                "{} {}",
                target.get(0..31).unwrap(),
                target.get(31..).unwrap()
            );
            println!(
                "{} {}",
                current_guess.get(0..31).unwrap(),
                current_guess.get(31..).unwrap()
            );

             */
            return Some(byte);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{get_cipher_text, recover_byte};
    use ascii::IntoAsciiString;

    #[test]
    fn capture_the_flag() {
        let number_of_blocks = 3;
        let block_size = 16;
        let mut plaintext = "".to_string();
        let mut append = "00".to_string().repeat(16);
        for _i in 0..(number_of_blocks * block_size) {
            append = append.get(2..).unwrap().to_string();
            plaintext.push_str(
                &recover_byte(
                    get_cipher_text(append.clone()),
                    append.clone() + &*plaintext,
                )
                .unwrap(),
            );
            println!(
                "{:?}",
                hex::decode(plaintext.clone()).unwrap().into_ascii_string()
            );
        }
    }
}
