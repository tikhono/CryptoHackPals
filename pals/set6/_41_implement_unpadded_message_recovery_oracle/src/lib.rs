#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;

    #[test]
    fn capture_the_flag() {
        let p = BigInt::parse_bytes(b"857504083339712752489993810777", 10).unwrap();
        let q = BigInt::parse_bytes(b"1029224947942998075080348647219", 10).unwrap();
        let phi: BigInt = (p.clone() - 1) * (q.clone() - 1);
        let e = BigInt::from(65537);
        let d = mod_inverse(e.clone(), phi).unwrap();
        let c = BigInt::parse_bytes(
            b"77578995801157823671636298847186723593814843845525223303932",
            10,
        )
        .unwrap();
        let n = p * q;

        println!("{}", c.modpow(&d, &n));

        let s = BigInt::from(11);

        let new_c = (s.modpow(&e, &n) * c) % n.clone();

        let new_p = new_c.modpow(&d, &n);

        println!("{}", (mod_inverse(s, n.clone()).unwrap() * new_p) % n);
    }
}
