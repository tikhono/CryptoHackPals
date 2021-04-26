#[cfg(test)]
mod tests {
    use _02_fixed_xor::fixed_xor;
    use ascii::IntoAsciiString;

    #[test]
    fn capture_the_flag() {
        let ciphertext = hex::decode(
            "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104",
        )
        .unwrap();
        let key = fixed_xor(
            &hex::encode(std::str::from_utf8(&ciphertext[0..8]).unwrap()),
            &hex::encode("crypto{1".to_string()),
        );
        println!(
            "{:?}",
            hex::decode(key.clone())
                .unwrap()
                .into_ascii_string()
                .unwrap()
        );
        println!(
            "{:?}",
            hex::decode(fixed_xor(
                &hex::encode(std::str::from_utf8(&ciphertext).unwrap()),
                &key.repeat(ciphertext.len() / key.len() + ciphertext.len() % key.len())
            ))
            .unwrap()
            .into_ascii_string()
            .unwrap()
        );
    }
}
