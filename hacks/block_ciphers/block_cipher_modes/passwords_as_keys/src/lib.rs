#[cfg(test)]
mod tests {
    use crypto::digest::Digest;
    use crypto::md5;
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter};

    #[test]
    fn it_works() {
        use std::io::Write;

        use curl::easy::Easy;
        let file_read = File::open("/home/vorrol/Projects/CryptoHackPals/hacks/block_ciphers/block_cipher_modes/passwords_as_keys/md5.txt").expect("file not found!");
        let reader = BufReader::new(file_read);
        let file_write =  File::create("/home/vorrol/Projects/CryptoHackPals/hacks/block_ciphers/block_cipher_modes/passwords_as_keys/out.txt").expect("couldn't create!");
        let mut writer = BufWriter::new(file_write);
        let addr = "http://aes.cryptohack.org/passwords_as_keys/decrypt/c92b7734070205bdf6c0087a751466ec13ae15e6f1bcdd3f3a535ec0f4bbae66/".to_string();
        for (i, line) in reader.lines().enumerate() {
            let mut buf = Vec::new();
            let mut encryptor = md5::Md5::new();
            encryptor.input_str(line.unwrap().as_str());
            let mut easy = Easy::new();
            let mut url = addr.clone();
            url.push_str(&*encryptor.result_str());
            url.push_str("/");
            easy.url(&*url).unwrap();
            {
                let mut transfer = easy.transfer();
                transfer
                    .write_function(|data| {
                        buf.extend_from_slice(data);
                        Ok(data.len())
                    })
                    .unwrap();
                transfer.perform().unwrap();
            }
            let string = String::from_utf8(buf).unwrap();
            let string = string.get(14..78).unwrap();
            let string = String::from_utf8(hex::decode(string).unwrap())
                .unwrap_or("unprintable".to_string());
            println!("{:?}   {:?}", i, string);
            if string != "unprintable" {
                writer
                    .write_all(string.as_ref())
                    .expect("error on writing to a file");
                writer
                    .write_all(i.to_string().as_bytes())
                    .expect("error on writing to a file");
            }
        }
    }
}
