#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;
    use num::integer::lcm;

    #[test]
    fn capture_the_flag() {
        let p = BigInt::parse_bytes(b"857504083339712752489993810777", 10).unwrap();
        let q = BigInt::parse_bytes(b"1029224947942998075080348647219", 10).unwrap();
        let phi: BigInt = lcm(p - 1, q - 1);
        let e = BigInt::from(65537);

        println!("{}", mod_inverse(e, phi).unwrap());
    }
}
