#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;

    #[test]
    fn capture_the_flag() {
        let p = BigInt::parse_bytes(b"857504083339712752489993810777", 10).unwrap();
        let q = BigInt::parse_bytes(b"1029224947942998075080348647219", 10).unwrap();
        let phi: BigInt = (&p - 1) * (&q - 1);
        let e = BigInt::from(17);
        let d = mod_inverse(e.clone(), phi).unwrap();
        let n = p * q;

        let plaintext = BigInt::parse_bytes(b"123", 10).unwrap();
        let ciphertext = plaintext.modpow(&e, &n);
        assert_eq!(plaintext, ciphertext.modpow(&d, &n));
    }
}
