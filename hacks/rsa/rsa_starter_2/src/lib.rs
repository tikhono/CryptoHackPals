#[cfg(test)]
mod tests {
    use num::bigint::BigInt;

    #[test]
    fn capture_the_flag() {
        let p = BigInt::from(17);
        let q = BigInt::from(23);
        let e = BigInt::from(65537);
        let x = BigInt::from(12);

        println!("{}", x.modpow(&e, &(p * q)));
    }
}
