#[cfg(test)]
mod tests {
    use _02_fixed_xor::fixed_xor;
    use ascii::IntoAsciiString;
    use ecb_cbc_wtf::get_response;

    #[test]
    fn capture_the_flag() {
        let addr = "http://aes.cryptohack.org/ctrime/encrypt/".to_string()
            + &*"63727970746f7b430102"
            + "/";
        let ciphertext = get_response(addr);
        let target_len = ciphertext.len();
        for i in 0x20..=0x7E {
            println!("{}", i);
            for j in 0x20..=0x7E {
                let addr = "http://aes.cryptohack.org/ctrime/encrypt/63727970746f7b43".to_string()
                    + &*hex::encode([i])
                    + &*hex::encode([j])
                    + "/";
                let ciphertext = get_response(addr);
                if ciphertext.len() != target_len {
                    println!("i is {}, j is {}, len is {}", i, j, ciphertext.len());
                }
            }
        }

        println!(
            "{}",
            hex::decode(ciphertext)
                .unwrap()
                .into_ascii_string()
                .unwrap()
        );
    }
}
