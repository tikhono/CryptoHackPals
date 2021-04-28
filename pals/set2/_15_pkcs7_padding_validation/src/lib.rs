pub fn validate_padding(data: &[u8]) -> bool {
    let pad_size = data.last().unwrap();

    for byte in data.iter().skip(data.len() - *pad_size as usize) {
        if *byte != *pad_size {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::validate_padding;
    use _09_implement_pkcs7_padding::pad;

    #[test]
    fn test_valid_pad_yellow_submarine_20() {
        assert_eq!(
            validate_padding(
                std::str::from_utf8(&pad(&"YELLOW SUBMARINE".as_bytes().to_vec(), 20))
                    .unwrap()
                    .as_ref()
            ),
            true
        );
    }

    #[test]
    fn test_invalid_pad_yellow_submarine_20() {
        assert_eq!(validate_padding(b"YELLOW SUBMARINE\x04\x04\x04"), false);
    }
}
