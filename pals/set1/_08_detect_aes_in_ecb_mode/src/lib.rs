#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn capture_the_flag() {
        let file_read = File::open("../../../pals/set1/_08_detect_aes_in_ecb_mode/ciphertexts.txt")
            .expect("file not found!");
        let reader = BufReader::new(file_read);

        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut map = HashMap::new();

            for block in line.as_bytes().chunks(32) {
                match map.get(block) {
                    None => {
                        map.insert(block, 0);
                    }
                    Some(value) => {
                        // Couldn't use value directly, getting warning on borrowing
                        // https://github.com/rust-lang/rust/issues/59159
                        // To see warning uncomment next line and comment next 2 after it
                        // map.insert(block, value + 1);
                        let new_val = value + 1;
                        map.insert(block, new_val);
                    }
                }
            }
            if map.values().sum::<u32>() != 0 {
                println!("{} {}", map.values().sum::<u32>(), line)
            }
            for (key, val) in map.iter() {
                if *val != 0 {
                    println!("{:?}", std::str::from_utf8(key).unwrap());
                }
            }
        }
    }
}
