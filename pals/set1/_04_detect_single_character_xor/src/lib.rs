#[cfg(test)]
mod tests {
    use _03_single_byte_xor_cipher::decode_single_byte_xor;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn capture_the_flag() {
        let file_read =
            File::open("../../../pals/set1/_04_detect_single_character_xor/ciphertexts.txt")
                .expect("file not found!");
        let reader = BufReader::new(file_read);
        for (i, line) in reader.lines().enumerate() {
            let plaintexts = decode_single_byte_xor(line.unwrap());
            for plaintext in plaintexts {
                println!("{}  |{}|", i, plaintext);
            }
        }
    }
}
