#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;

    #[test]
    fn capture_the_flag() {
        let e = BigInt::from(3);

        let p = BigInt::from(263);
        let q = BigInt::from(991);
        let r = BigInt::from(191);

        let n1 = &p * &q;
        let n2 = &p * &r;
        let n3 = &q * &r;

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
