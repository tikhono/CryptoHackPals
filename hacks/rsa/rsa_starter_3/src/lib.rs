#[cfg(test)]
mod tests {
    use num::bigint::BigInt;

    #[test]
    fn capture_the_flag() {
        let p = BigInt::parse_bytes(b"857504083339712752489993810777", 10).unwrap();
        let q = BigInt::parse_bytes(b"1029224947942998075080348647219", 10).unwrap();

        println!("{}", &((p - 1) * (q - 1)));
    }
}
