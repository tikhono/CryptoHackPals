#[cfg(test)]
mod tests {
    use openssl::bn::BigNum;
    use openssl::dsa::Dsa;
    use openssl::hash::MessageDigest;
    use openssl::pkey::PKey;
    use openssl::sign::{Signer, Verifier};

    #[test]
    fn capture_the_flag() {
        let g = BigNum::from_u32(0).unwrap();
        let priv_key = BigNum::from_u32(3).unwrap();
        let p = BigNum::from_u32(17).unwrap();
        let q = BigNum::from_u32(19).unwrap();
        let mut pub_key = BigNum::new().unwrap();
        pub_key
            .mod_exp(
                &g,
                &priv_key,
                &p,
                &mut *openssl::bn::BigNumContext::new().unwrap(),
            )
            .unwrap();
        let dsa = Dsa::from_private_components(p, q, g, priv_key, pub_key).unwrap();

        let pkey = PKey::from_dsa(dsa).unwrap();

        let data = b"hello, world!";
        let data2 = b"hola, mundo!";

        // Sign the data
        let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
        signer.update(data).unwrap();
        signer.update(data2).unwrap();
        let signature = signer.sign_to_vec().unwrap();

        // Verify the data
        let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
        verifier.update(data).unwrap();
        verifier.update(data2).unwrap();
        assert!(verifier.verify(&signature).unwrap());
    }
}
