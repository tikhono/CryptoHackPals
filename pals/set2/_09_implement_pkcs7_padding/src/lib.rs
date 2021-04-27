pub fn pad(data: &Vec<u8>, block_size: u8) -> Vec<u8> {
    let pad_size = block_size - data.len() as u8 % block_size;
    let mut padded = data.clone();

    for _i in 0..pad_size {
        padded.push(pad_size);
    }
    padded
}

#[cfg(test)]
mod tests {
    use crate::pad;

    #[test]
    fn test_pad_yellow_submarine_20() {
        assert_eq!(
            std::str::from_utf8(&pad(&"YELLOW SUBMARINE".as_bytes().to_vec(), 20)).unwrap(),
            "YELLOW SUBMARINE\x04\x04\x04\x04"
        );
    }
}
