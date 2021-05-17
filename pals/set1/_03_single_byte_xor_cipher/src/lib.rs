pub fn single_byte_xor(ciphertext: String, key: char) -> String {
    use _02_fixed_xor::fixed_xor;

    fixed_xor(
        &ciphertext,
        &hex::encode(key.to_string().repeat(ciphertext.len())),
    )
}

pub fn score_text(text: String) -> u8 {
    use ascii::IntoAsciiString;
    use whatlang::{detect, Lang};

    let text = hex::decode(text).unwrap();
    let text = match text.into_ascii_string() {
        Ok(ascii) => ascii,
        Err(_) => return 0,
    };

    for i in 0..text.len() {
        if !(text[i].is_ascii_printable() || text[i] == ascii::AsciiChar::LineFeed) {
            return 0;
        }
    }
    let info = detect(text.as_ref()).unwrap();
    if info.lang() == Lang::Eng && info.confidence() >= 0.5 {
        (info.confidence().round() * 100.0) as u8
    } else {
        0
    }
}

pub fn decode_single_byte_xor(ciphertext: String) -> Vec<String> {
    use ascii::IntoAsciiString;

    let mut decoded: Vec<String> = Vec::new();
    for i in 0u8..=255 {
        let text = single_byte_xor(ciphertext.clone(), i as char);
        if score_text(text.clone()) != 0 {
            decoded.push(
                hex::decode(text)
                    .unwrap()
                    .into_ascii_string()
                    .unwrap()
                    .to_string(),
            );
        }
    }
    decoded
}

#[cfg(test)]
mod tests {
    use crate::decode_single_byte_xor;

    #[test]
    fn capture_the_flag() {
        let plaintexts = decode_single_byte_xor(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string(),
        );
        for plaintext in plaintexts {
            println!("{}", plaintext);
        }
    }
}
