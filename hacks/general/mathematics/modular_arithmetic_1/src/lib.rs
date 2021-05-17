#[cfg(test)]
mod tests {
    use num::bigint::BigInt;
    use std::cmp::min;
    use std::ops::Rem;

    #[test]
    fn capture_the_flag() {
        assert_eq!(
            BigInt::from(4),
            min(
                BigInt::from(11 % 6),
                BigInt::parse_bytes(b"8146798528947", 10)
                    .unwrap()
                    .rem(BigInt::from(17))
            )
        );
        println!(
            "{:?}",
            min(
                BigInt::from(11 % 6),
                BigInt::parse_bytes(b"8146798528947", 10)
                    .unwrap()
                    .rem(BigInt::from(17))
            )
        );
    }
}
