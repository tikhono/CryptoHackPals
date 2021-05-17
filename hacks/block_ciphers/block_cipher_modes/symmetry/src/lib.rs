#[cfg(test)]
mod tests {
    use ascii::IntoAsciiString;

    #[test]
    fn capture_the_flag() {
        use curl::easy::Easy;
        let mut data = Vec::new();
        let mut handle = Easy::new();
        let addr = "http://aes.cryptohack.org/symmetry/encrypt_flag/".to_string();
        handle.url(&addr).unwrap();
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
        let string = std::str::from_utf8(&data)
            .unwrap()
            .strip_prefix("{\"ciphertext\":\"")
            .unwrap()
            .strip_suffix("\"}\n")
            .unwrap()
            .to_string();
        let iv = string.get(0..32).unwrap();
        let ciphertext = string.get(32..).unwrap();
        let mut addr = "http://aes.cryptohack.org/symmetry/encrypt/".to_string();
        addr.push_str(ciphertext);
        addr.push_str("/");
        addr.push_str(iv);
        addr.push_str("/");
        let mut data = Vec::new();

        handle.url(&addr).unwrap();
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
        let string = std::str::from_utf8(&data)
            .unwrap()
            .strip_prefix("{\"ciphertext\":\"")
            .unwrap()
            .strip_suffix("\"}\n")
            .unwrap()
            .to_string();
        println!("{:?}", hex::decode(string).unwrap().into_ascii_string());
    }
}
