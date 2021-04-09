#[cfg(test)]
mod tests {
    use _02_fixed_xor::fixed_xor;
    use ecb_cbc_wtf::get_response;

    #[test]
    fn capture_the_flag() {
        let addr = "http://aes.cryptohack.org/flipping_cookie/get_cookie/".to_string();
        let ciphertext = get_response(addr);

        let iv = ciphertext.get(..32).unwrap();
        let cookie = ciphertext.get(32..).unwrap();
        let target_text = fixed_xor(
            &*hex::encode("admin=True;;;;;;"),
            &*hex::encode("admin=False;;;;;"),
        );
        let new_iv = fixed_xor(iv, &*target_text);

        let addr = "http://aes.cryptohack.org/flipping_cookie/check_admin/".to_string()
            + cookie
            + "/"
            + &*new_iv
            + "/";
        let plaintext = get_response(addr);
        println!("{}", plaintext);
    }
}
