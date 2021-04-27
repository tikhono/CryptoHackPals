mod tests {

    #[test]
    fn capture_the_flag() {
        use ascii::IntoAsciiString;
        use ecb_cbc_wtf::get_response;
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();

        for _i in 0..200 {
            let addr = "http://aes.cryptohack.org/stream_consciousness/encrypt/".to_string();
            let ciphertext = get_response(addr);

            map.insert(ciphertext.len() / 2, ciphertext);
        }
        for ciph in map {
            print!("{:?}\t", ciph.0);
            for chunk in ciph.1.as_bytes().chunks(32) {
                print!("{:?}", chunk.into_ascii_string().unwrap());
            }
            println!();
        }
    }
}
