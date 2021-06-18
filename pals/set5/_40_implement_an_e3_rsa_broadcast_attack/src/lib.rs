#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::{BigInt, Sign};
    use rsa::{BigUint, PublicKeyParts};

    #[test]
    fn capture_the_flag() {
        use rand::rngs::OsRng;
        use rsa::{RSAPrivateKey, RSAPublicKey};
        let mut rng = OsRng;
        let bits = 512;
        let private_key_1 = RSAPrivateKey::new_with_exp(&mut rng, bits, &BigUint::from(3u8))
            .expect("failed to generate a key");
        let public_key_1 = RSAPublicKey::from(&private_key_1);
        let private_key_2 = RSAPrivateKey::new_with_exp(&mut rng, bits, &BigUint::from(3u8))
            .expect("failed to generate a key");
        let public_key_2 = RSAPublicKey::from(&private_key_2);
        let private_key_3 = RSAPrivateKey::new_with_exp(&mut rng, bits, &BigUint::from(3u8))
            .expect("failed to generate a key");
        let public_key_3 = RSAPublicKey::from(&private_key_3);

        let e = BigInt::from(3);

        let n1 = BigInt::from_bytes_be(Sign::Plus, &public_key_1.n().to_bytes_be());
        let n2 = BigInt::from_bytes_be(Sign::Plus, &public_key_2.n().to_bytes_be());
        let n3 = BigInt::from_bytes_be(Sign::Plus, &public_key_3.n().to_bytes_be());

        let plaintext = BigInt::from(12345);

        let ciphertext1 = plaintext.modpow(&e, &n1);
        let ciphertext2 = plaintext.modpow(&e, &n2);
        let ciphertext3 = plaintext.modpow(&e, &n3);

        let p1 = &ciphertext1 * &n2 * &n3 * mod_inverse(&n2 * &n3, n1.clone()).unwrap();
        let p2 = &ciphertext2 * &n1 * &n3 * mod_inverse(&n1 * &n3, n2.clone()).unwrap();
        let p3 = &ciphertext3 * &n2 * &n1 * mod_inverse(&n2 * &n1, n3.clone()).unwrap();

        let res = (p1 + p2 + p3) % (n1 * n2 * n3);

        assert_eq!(plaintext, res.nth_root(3));
    }
}
