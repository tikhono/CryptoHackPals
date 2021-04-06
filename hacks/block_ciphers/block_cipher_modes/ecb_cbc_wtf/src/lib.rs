pub fn get_response(addr: String) -> String {
    use curl::easy::Easy;
    let mut data = Vec::new();
    let mut handle = Easy::new();
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
    let result = std::str::from_utf8(&*data)
        .unwrap()
        .strip_prefix("{\"")
        .unwrap()
        .strip_suffix("\"}\n")
        .unwrap()
        .to_string();
    let result: Vec<&str> = result.split("\":\"").collect();
    result[1].to_string()
}

#[cfg(test)]
mod tests {
    use crate::get_response;
    use _02_fixed_xor::fixed_xor;
    use ascii::IntoAsciiString;

    #[test]
    fn capture_the_flag() {
        let addr = "http://aes.cryptohack.org/ecbcbcwtf/encrypt_flag/".to_string();
        let ciphertext = get_response(addr);

        let iv = ciphertext.get(0..32).unwrap();
        let ciphertext1 = ciphertext.get(32..64).unwrap();
        let ciphertext2 = ciphertext.get(64..).unwrap();

        let addr = "http://aes.cryptohack.org/ecbcbcwtf/decrypt/".to_string() + ciphertext1 + "/";
        let derive_plaintext_1 = get_response(addr);

        let addr = "http://aes.cryptohack.org/ecbcbcwtf/decrypt/".to_string() + ciphertext2 + "/";
        let derive_plaintext_2 = get_response(addr);

        let plaintext_1 = fixed_xor(&*derive_plaintext_1, iv);
        let plaintext_2 = fixed_xor(&*derive_plaintext_2, ciphertext1);

        println!(
            "{}",
            hex::decode(plaintext_1 + &*plaintext_2)
                .unwrap()
                .into_ascii_string()
                .unwrap()
        );
    }
}
