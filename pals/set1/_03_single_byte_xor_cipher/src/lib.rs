pub fn single_byte_xor(ciphertext: String, key: char) -> String {
    use _02_fixed_xor::fixed_xor;

    fixed_xor(
        &*ciphertext,
        &*hex::encode(key.to_string().repeat(ciphertext.len())),
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
        if !text[i].is_ascii_printable() {
            return 0;
        }
    }
    let info = detect(text.as_ref()).unwrap();
    if info.lang() == Lang::Eng {
        (info.confidence() * 100.0) as u8
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{score_text, single_byte_xor};

    #[test]
    fn capture_the_flag() {
        for i in 0u8..=255 {
            if score_text(single_byte_xor(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string(),
                i as char,
            )) == 0
            {
                continue;
            }

            println!(
                "Key is {:#010b} and plaintext is {:?}",
                i,
                std::str::from_utf8(
                    &*hex::decode(single_byte_xor(
                        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
                            .to_string(),
                        i as char
                    ))
                    .unwrap()
                )
                .unwrap()
            );
        }
    }
}
