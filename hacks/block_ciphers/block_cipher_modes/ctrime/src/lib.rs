#[cfg(test)]
mod tests {
    use ascii::IntoAsciiString;
    use ecb_cbc_wtf::get_response;
    use std::io;
    use std::io::Write;

    #[test]
    fn capture_the_flag() {
        // assuming flag starts with "crypto{" lets try to figure out next bytes
        let mut plaintext = "63727970746f7b".to_string();

        for _ in 0..32 {
            let addr = "http://aes.cryptohack.org/ctrime/encrypt/".to_string()
                + plaintext.get(plaintext.len() - 10..).unwrap()
                + "00/";
            let ciphertext = get_response(addr);
            let target_len = ciphertext.len();

            for byte in 0x20..=0x7E {
                //This is a range of printable ascii characters
                let addr = "http://aes.cryptohack.org/ctrime/encrypt/".to_string()
                    + plaintext.get(plaintext.len() - 10..).unwrap()
                    + &hex::encode([byte])
                    + "/";
                let ciphertext = get_response(addr);
                if ciphertext.len() != target_len {
                    plaintext.push_str(hex::encode([byte]).as_str());
                    break;
                }
            }
            if plaintext.ends_with(&hex::encode("}")) {
                break;
            }
        }
        println!(
            "{}",
            hex::decode(plaintext)
                .expect("Failed to decode hex plaintext")
                .into_ascii_string()
                .expect("Failed to convert into ascii string")
        );
    }
}
